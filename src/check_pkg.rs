use crate::{
    cache::{self, MprPackage},
    pkglist, util,
};
use git2::{BranchType, IndexAddOption, ObjectType, Remote, Repository, Signature};
use octocrab::{
    models::{
        workflows::{Conclusion, Status},
        RunId,
    },
    params::State,
};
use regex::Regex;
use serde_json::{json, Value};
use std::{
    fs::{self, File},
    io::{self, prelude::*},
    path::Path,
    process::Command,
    thread,
    time::Duration,
};

/// The GitHub Actions package updater workflow file. See the code usages in
/// this file for more info.
static PKG_UPDATE_ACTION: &[u8] = include_bytes!("../.github/workflows/update-pkg.yml");

/// The CODEOWNERS file we use to review the proper people on package updates.
static PKG_CODEOWNERS: &[u8] = include_bytes!("actions/CODEOWNERS");

/// The supported distros on the Prebuilt-MPR.
static SUPPORTED_DISTROS: [&str; 5] = ["focal", "jammy", "lunar", "bullseye", "bookworm"];

/// The supported architectures on the Prebuilt-MPR.
static SUPPORTED_ARCHITECTURES: [&str; 2] = ["amd64", "arm64"];

pub async fn check_pkg(gh_token: &str, pkg: &str) -> exitcode::ExitCode {
    let packages = match cache::get_mpr_packages().await {
        Ok(pkgs) => pkgs,
        Err(err) => {
            log::error!("Unable to fetch package list from the MPR: {err:?}");
            return exitcode::TEMPFAIL;
        }
    };
    let package = match packages.iter().find(|json_pkg| json_pkg.pkgbase == pkg) {
        Some(package) => package,
        None => {
            log::error!("The package '{pkg}' couldn't be found on the MPR.");
            return exitcode::DATAERR;
        }
    };

    // The distros/architectures that the package needs to be built for.
    // Stored as tuples of (`distro`, `arch`).
    let pkg_config = pkglist::PKGLIST.get(pkg).unwrap();
    let mut old_version = None;
    let mut needed_updates = vec![];

    // Check each distro/architecture for updates.
    for arch in SUPPORTED_ARCHITECTURES {
        // This regex finds the version in a package line, while also validating that a
        // package line can be found.
        let version_re = {
            let re = format!("^{pkg}/[a-z,]* ([a-z0-9.:-]+) {arch}");
            Regex::new(&re).unwrap()
        };

        for distro in SUPPORTED_DISTROS {
            if let Err(err) = fs::write(
                "/etc/apt/sources.list",
                format!("deb [arch=all,{arch} signed-by=/usr/share/keyrings/prebuilt-mpr-archive-keyring.gpg] https://{} prebuilt-mpr {distro}", util::PROGET_URL)
            ) {
                log::error!("Failed to write Prebuilt-MPR sources.list file: {err}");
                return exitcode::TEMPFAIL;
            };
            let output = Command::new("apt-get").arg("update").output().unwrap();
            if !output.status.success() {
                io::stderr().write_all(&output.stderr).unwrap();
                log::error!("Failed to update APT cache");
                return exitcode::TEMPFAIL;
            }

            let output = Command::new("apt").args(["list", pkg]).output().unwrap();
            if !output.status.success() {
                io::stderr().write_all(&output.stderr).unwrap();
                log::error!("Failed to fetch version list for '{pkg}'");
                return exitcode::TEMPFAIL;
            }
            let stdout = String::from_utf8(output.stdout).unwrap();
            let version = stdout
                .lines()
                .find(|line| version_re.is_match(line))
                .map(|line| version_re.captures(line).unwrap()[1].to_owned());

            if let Some(ver) = &version {
                old_version = Some(ver.to_owned());
            }

            if (version.is_none() || version.as_ref().unwrap() != &package.version)
                && !pkg_config.blocked_distros.contains(&distro.to_owned())
                && !pkg_config.blocked_archs.contains(&arch.to_owned())
            {
                needed_updates.push((distro, arch));
            }
        }
    }

    // If we have package versions that need to be updated, then do so.
    if !needed_updates.is_empty() {
        log::info!("Updating '{pkg}'...");
        return update_pkg(gh_token, package, old_version, &needed_updates).await;
    } else {
        log::info!("'{pkg}' is up to date!");
    }

    exitcode::OK
}

async fn update_pkg(
    gh_token: &str,
    pkg: &MprPackage,
    old_version: Option<String>,
    needed_updates: &[(&str, &str)],
) -> exitcode::ExitCode {
    // Stuff we need throughout this function.
    let gh_repo_url = format!(
        "https://x-access-token:{gh_token}@github.com/{}/{}",
        util::PBMPR_GITHUB_ORG,
        util::PBMPR_GITHUB_REPO
    );
    let mpr_repo_url = format!("https://{}/{}", util::MPR_URL, pkg.pkgbase);
    let gh_pkg_branch = format!("pkg/{}", pkg.pkgbase);
    let gh_pkg_update_branch = format!("pkg-update/{}", pkg.pkgbase);

    // Set up the octocrab instance.
    let crab = octocrab::instance();
    let workflows = crab.workflows(util::PBMPR_GITHUB_ORG, util::PBMPR_GITHUB_REPO);
    let issues = crab.issues(util::PBMPR_GITHUB_ORG, util::PBMPR_GITHUB_REPO);

    // Clone the GitHub and MPR repos.
    log::info!("Cloning '{mpr_repo_url}' into 'mpr-repo/'...");
    let mpr_repo = Repository::clone(&mpr_repo_url, "mpr-repo").unwrap();
    log::info!(
        "Cloning 'https://github.com/{}/{}' into 'gh-repo/'...",
        util::PBMPR_GITHUB_ORG,
        util::PBMPR_GITHUB_REPO
    );
    let gh_repo = Repository::clone(&gh_repo_url, "gh-repo").unwrap();
    let mut gh_remote = gh_repo.find_remote("origin").unwrap();

    // Get the list of branches on the GitHub repo.
    let remote_gh_branches = util::get_branch_names(&gh_repo, BranchType::Remote);

    // Create and push the needed branches if they don't exist yet.
    log::info!("Ensuring Git branches are in a good state...");

    // Create and push the 'pkg/{pkg}' and 'pkg-update/{pkg}' branches if they don't
    // exist.
    for branch_name in [&gh_pkg_branch, &gh_pkg_update_branch] {
        let remote_branch = format!("origin/{branch_name}");

        // If the branch existed on the remote, configure the local branch to point to
        // the remote branch.
        if remote_gh_branches.contains(branch_name) {
            let commit = gh_repo
                .find_branch(&remote_branch, BranchType::Remote)
                .unwrap()
                .into_reference()
                .peel_to_commit()
                .unwrap();
            let mut branch = gh_repo.branch(branch_name, &commit, false).unwrap();
            branch.set_upstream(Some(&remote_branch)).unwrap();
            continue;
        }

        // Otherwise create the branch.
        let tree = {
            let mut index = gh_repo.index().unwrap();
            index.clear().unwrap();
            let tree_id = index.write_tree().unwrap();
            gh_repo.find_tree(tree_id).unwrap()
        };

        // If we're creating the 'pkg/{pkg}' branch, create a new commit.
        let commit = if branch_name == &gh_pkg_branch {
            let signature = util::git_signature();
            let commit_id = gh_repo
                .commit(None, &signature, &signature, "Initial commit", &tree, &[])
                .unwrap();
            gh_repo.find_commit(commit_id).unwrap()
        // Otherwise we're creating the 'pkg-update/{pkg}' branch, and need to
        // make the first commit the same as the one from 'pkg/{pkg}'.
        } else {
            gh_repo
                .find_branch(&gh_pkg_branch, BranchType::Local)
                .unwrap()
                .into_reference()
                .peel_to_commit()
                .unwrap()
        };

        let mut branch = gh_repo.branch(branch_name, &commit, false).unwrap();
        let branch_ref = gh_repo
            .resolve_reference_from_short_name(branch_name)
            .unwrap()
            .name()
            .unwrap()
            .to_owned();
        gh_remote.push(&[&branch_ref], None).unwrap();
        branch.set_upstream(Some(&remote_branch)).unwrap();
    }

    check_actions_file(&gh_repo, &mut gh_remote, &gh_pkg_branch);

    // Checkout the GitHub repository to the correct branch.
    let gh_branch = gh_repo
        .resolve_reference_from_short_name(&gh_pkg_update_branch)
        .unwrap();
    gh_repo
        .checkout_tree(&gh_branch.peel(ObjectType::Any).unwrap(), None)
        .unwrap();
    gh_repo.set_head(gh_branch.name().unwrap()).unwrap();

    // Checkout the MPR repository to the correct tag.
    //
    // Git Tags on the MPR have epochs (:) replaced with exclamation marks (!), so
    // do that here.
    let mpr_tag_string = format!("ver/{}", pkg.version.replace(':', "!"));
    let mpr_tag = mpr_repo
        .resolve_reference_from_short_name(&mpr_tag_string)
        .unwrap();
    mpr_repo
        .checkout_tree(&mpr_tag.peel(ObjectType::Any).unwrap(), None)
        .unwrap();
    mpr_repo.set_head(mpr_tag.name().unwrap()).unwrap();

    // Delete all files from this branch.
    for maybe_file in fs::read_dir("gh-repo/").unwrap() {
        let file = maybe_file.unwrap();
        let file_type = file.file_type().unwrap();
        let file_path = file.path();

        if file_path.file_name().unwrap() == Path::new(".git") {
            continue;
        } else if file_type.is_dir() {
            fs::remove_dir_all(file_path).unwrap();
        } else {
            fs::remove_file(file_path).unwrap();
        }
    }

    // Copy over the files from the MPR repository into the GitHub branch's folder.
    log::info!("Setting up package's GitHub branch with files from the MPR repository...");
    let mut file_list = vec![];
    for maybe_file in fs::read_dir("mpr-repo/").unwrap() {
        let file = maybe_file.unwrap();
        let file_name = file.file_name().into_string().unwrap();

        if [".git", ".github", ".SRCINFO"].contains(&file_name.as_str()) {
            continue;
        }

        fs::copy(file.path(), format!("gh-repo/{}", file_name)).unwrap();
        file_list.push(file_name);
    }

    // Add the new files into the GitHub branch.
    let mut gh_index = gh_repo.index().unwrap();
    gh_index
        .add_all(&file_list, IndexAddOption::DEFAULT, None)
        .unwrap();
    gh_index.write().unwrap();

    // The number of changes, minus any changes to the '.github/' directory. For
    // some reason Git likes to register the lack of the '.github/' as a
    // deletion from that directory sometimes, but we don't want to include
    // those files as part of the changes.
    let status_count = gh_repo
        .statuses(None)
        .unwrap()
        .into_iter()
        .filter(|status| !status.path().unwrap().starts_with(".github"))
        .count();

    // Commit and push the new files into the the GitHub branch, if anything was
    // changed.
    if status_count == 0 {
        log::info!("GitHub repo already has changes on remote. Skipping pushing of changes.");
    } else {
        let signature = Signature::now(util::GIT_NAME, util::GIT_EMAIL).unwrap();
        let gh_tree = {
            let id = gh_index.write_tree().unwrap();
            gh_repo.find_tree(id).unwrap()
        };
        let prev_commit = {
            let id = gh_repo.refname_to_id("HEAD").unwrap();
            gh_repo.find_commit(id).unwrap()
        };
        gh_repo
            .commit(
                Some("HEAD"),
                &signature,
                &signature,
                &format!("Update package version to '{}'", pkg.version),
                &gh_tree,
                &[&prev_commit],
            )
            .unwrap();
        let gh_pkg_update_branch_ref = gh_repo
            .resolve_reference_from_short_name(&gh_pkg_update_branch)
            .unwrap()
            .name()
            .unwrap()
            .to_owned();
        gh_remote.push(&[&gh_pkg_update_branch_ref], None).unwrap();
    }

    // Check the latest workflow run to see if it's building the correct packages.
    // If it's not, cancel it and start a new run.
    //
    // First get the list of workflow runs.
    let mut workflow_run = None;
    let mut workflow_list = vec![];

    let mut page_num: u32 = 0;

    loop {
        let mut new_item = false;
        let page = match workflows
            .list_runs("update-pkg.yml")
            .branch(&gh_pkg_update_branch)
            .page(page_num)
            .send()
            .await
            .ok()
        {
            Some(page) => page,
            None => break,
        };

        for new_run in page.items {
            new_item = true;
            if !workflow_list.iter().any(|run| run == &new_run) {
                workflow_list.push(new_run);
            }
        }

        if !new_item {
            break;
        }

        page_num += 1;
    }

    // Next, check for old running jobs (anything but the most recent job) and
    // cancel them.
    if workflow_list.len() > 1 {
        for workflow in &workflow_list[1..] {
            let status: Status =
                serde_json::from_str(&format!(r#"{:?}"#, workflow.status)).unwrap();
            if status == Status::Queued || status == Status::InProgress {
                log::info!("Cancelling old workflow run '#{}'", workflow.run_number);
                crab.actions()
                    .cancel_workflow_run(
                        util::PBMPR_GITHUB_ORG,
                        util::PBMPR_GITHUB_REPO,
                        workflow.id,
                    )
                    .await
                    .unwrap();
            }
        }
    }

    // Next, check the latest run for the original required checks.
    if !workflow_list.is_empty() {
        workflow_run = Some(workflow_list.swap_remove(0));
    }
    let correct_jobs = async {
        if let Some(run) = workflow_run.as_ref() {
            let actual_jobs: Vec<String> = workflows
                .list_jobs(run.id)
                .send()
                .await
                .unwrap()
                .items
                .into_iter()
                .map(|item| item.name)
                .collect();
            let mut expected_jobs = vec![];

            for target in needed_updates {
                // The format of this string is closely tied to that from
                // `src/actions/update-pkg.yml` in this repository.
                expected_jobs.push(format!("Build Package ({}:{})", target.0, target.1));
            }

            for job in &expected_jobs {
                if !actual_jobs.contains(job) {
                    return false;
                }
            }

            let conclusion: Conclusion = serde_json::from_str(&format!(
                r#"{:?}"#,
                run.conclusion
                    .as_deref()
                    // Temporary fix for https://github.com/XAMPPRocky/octocrab/issues/422.
                    .map(|conclusion| if conclusion == "startup_failure" {
                        "failure"
                    } else {
                        conclusion
                    })
                    .unwrap_or("success")
            ))
            .unwrap();

            actual_jobs.len() == expected_jobs.len()
                && ![
                    Conclusion::Cancelled,
                    Conclusion::TimedOut,
                    Conclusion::Failure,
                ]
                .contains(&conclusion)
        } else {
            false
        }
    }
    .await;

    // Create a new job if:
    // 1. The newest job isn't building the correct packages, or
    // 2. The previous job failed, or
    // 3. There was no previous job.
    if !correct_jobs || workflow_run.is_none() {
        // Cancel the old job if it's still running.
        if let Some(run) = &workflow_run {
            let status: Status = serde_json::from_str(&format!(r#"{:?}"#, run.status)).unwrap();
            if status == Status::Queued || status == Status::InProgress {
                log::info!("Cancelling out of date job '#{}'...", run.run_number);
                crab.actions()
                    .cancel_workflow_run(util::PBMPR_GITHUB_ORG, util::PBMPR_GITHUB_REPO, run.id)
                    .await
                    .unwrap();
            }
        }

        // Create the new job.
        let mut targets = vec![];
        for target in needed_updates {
            targets.push(json!({
                "arch": target.1,
                "distro": target.0,
                "image-tag": match target.0 {
                    "focal" => "ubuntu-focal",
                    "jammy" => "ubuntu-jammy",
                    "lunar" => "ubuntu-lunar",
                    "bullseye" => "debian-bullseye",
                    "bookworm" => "debian-bookworm",
                    _ => unreachable!()
                }
            }));
        }

        crab.actions()
            .create_workflow_dispatch(
                util::PBMPR_GITHUB_ORG,
                util::PBMPR_GITHUB_REPO,
                "update-pkg.yml",
                &gh_pkg_update_branch,
            )
            .inputs(json!({
                "pkgbase": pkg.pkgbase,
                "targets": Value::Array(targets).to_string()
            }))
            .send()
            .await
            .unwrap();
        let new_job = loop {
            let mut tries = 0;
            let maybe_job = workflows
                .list_runs("update-pkg.yml")
                .branch(&gh_pkg_update_branch)
                .per_page(1)
                .send()
                .await
                .unwrap()
                .items
                .get(0)
                .map(|job| job.to_owned());

            // When we create a job it doesn't show up immediately. So loop until it does
            // (or bork out if we can't get it after a while).
            let prev_id = workflow_run.as_ref().map(|run| run.id).unwrap_or(RunId(0));
            if let Some(job) = maybe_job && job.id != prev_id {
                break job;
            } else {
                tries += 1;
                if tries == 10 {
                    log::error!("Failed to fetch new run job. Is it running?");
                    return exitcode::TEMPFAIL;
                }
                thread::sleep(Duration::from_secs(1));
            }
        };
        log::info!("Created new workflow run at '#{}'", new_job.run_number);
        workflow_run = Some(new_job);
    }

    // Check if there's a diff.
    let is_diff = !crab
        .get::<serde_json::Value, String, &str>(
            format!(
                "/repos/{}/{}/compare/{gh_pkg_branch}...{gh_pkg_update_branch}",
                util::PBMPR_GITHUB_ORG,
                util::PBMPR_GITHUB_REPO
            ),
            None,
        )
        .await
        .unwrap()["files"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|value| !value["filename"].as_str().unwrap().starts_with(".github/"))
        .collect::<Vec<_>>()
        .is_empty();

    // Generate the issue description.
    let run = workflow_run.unwrap();
    let issue_title = format!(
        "{}: `{}`",
        if old_version.is_none() {
            "New Package"
        } else {
            "Package Update"
        },
        pkg.pkgbase
    );
    let issue_desc = format!(
        "\
        `{}` has a new version available and is ready to be reviewed makedeb/prebuilt-mpr-reviewers!

        ## Package Information
        **Status**
        :building_construction: Building...

        **Update Job**
        [#{}](https://github.com/{}/{}/actions/runs/{})

        **Version**
        {}

        **Changed Files**
        {}

        **Updated Distributions**
        {}\
        ",
        pkg.pkgbase,
        run.run_number,
        util::PBMPR_GITHUB_ORG,
        util::PBMPR_GITHUB_REPO,
        run.id,
        if let Some(ver) = &old_version {
            if ver == &pkg.version {
                "*no change*".to_owned()
            } else {
                format!("`{ver}` ⭢ **`{}`**", pkg.version)
            }
        } else {
            format!("`{}`", pkg.version)
        },
        if is_diff {
            format!("[`{gh_pkg_branch}` ⭠ `{gh_pkg_update_branch}`](https://github.com/{0}/{1}/compare/{gh_pkg_branch}...{gh_pkg_update_branch})", util::PBMPR_GITHUB_ORG, util::PBMPR_GITHUB_REPO)
        } else {
            "*no files changed*".to_owned()
        },
        {
            let mut pkgs: Vec<(String, String)> = vec![];
            for pkg in needed_updates {
                match pkgs.iter().position(|item| item.0 == pkg.0) {
                    Some(index) => {
                        let arch_string = &mut pkgs[index].1;
                        if !arch_string.contains(pkg.1) {
                            arch_string.push_str(&format!(", `{}`", pkg.1))
                        }
                    },
                    None => pkgs.push((pkg.0.to_owned(), format!("`{}`", pkg.1)))
                }
            }

            pkgs.into_iter()
                .map(|item| format!("\\- {1} ({})", item.1, match item.0.as_str() {
                    "focal" => "Ubuntu 20.04",
                    "jammy" => "Ubuntu 22.04",
                    "lunar" => "Ubuntu 23.04",
                    "bullseye" => "Debian 11",
                    "bookworm" => "Debian 12",
                    _ => unreachable!()
                }))
                .collect::<Vec<String>>()
                .join("\n")
        }
    )
    .lines()
    .map(|line| line.trim_start())
    .collect::<Vec<&str>>()
    .join("\n");

    // If an old issue exists, update the title and body.
    let issue_list = issues
        .list()
        .per_page(100)
        .state(State::Open)
        .send()
        .await
        .unwrap();
    let issue_list = crab.all_pages(issue_list).await.unwrap();
    let maybe_issue = issue_list
        .iter()
        .find(|issue| issue.title.contains(&format!("`{}`", pkg.pkgbase)));

    if let Some(issue) = maybe_issue && (issue.title != issue_title || issue.body != Some(issue_desc.clone())) {
        log::info!("Updating issue #{}...", issue.number);
        issues
            .update(issue.number)
            .title(&issue_title)
            .body(&issue_desc)
            .send()
            .await
            .unwrap();
    } else if maybe_issue.is_none() {
        let issue = issues
            .create(issue_title)
            .body(issue_desc)
            .send()
            .await
            .unwrap();
        log::info!("Created new issue #{}", issue.number);
    }
    exitcode::OK
}

/// Make sure that the GitHub Actions file is created and up to date on the
/// package branch.
fn check_actions_file(gh_repo: &Repository, gh_remote: &mut Remote, pkg_branch: &str) {
    let gh_branch = gh_repo
        .resolve_reference_from_short_name(pkg_branch)
        .unwrap();
    gh_repo
        .checkout_tree(&gh_branch.peel(ObjectType::Any).unwrap(), None)
        .unwrap();
    gh_repo.set_head(gh_branch.name().unwrap()).unwrap();

    fs::create_dir_all("gh-repo/.github/workflows").unwrap();
    File::create("gh-repo/.github/workflows/update-pkg.yml")
        .unwrap()
        .write_all(PKG_UPDATE_ACTION)
        .unwrap();
    File::create("gh-repo/.github/CODEOWNERS")
        .unwrap()
        .write_all(PKG_CODEOWNERS)
        .unwrap();

    if !gh_repo.statuses(None).unwrap().is_empty() {
        log::info!(
            "Updating GitHub Actions workflow for package updates on '{pkg_branch}' branch..."
        );

        // Stage the files.
        let mut index = gh_repo.index().unwrap();
        index
            .add_all(
                [".github/workflows/update-pkg.yml", ".github/CODEOWNERS"],
                IndexAddOption::DEFAULT,
                None,
            )
            .unwrap();
        index.write().unwrap();

        // Make the commit.
        let tree = {
            let tree_id = index.write_tree().unwrap();
            gh_repo.find_tree(tree_id).unwrap()
        };
        let prev_commit = {
            let commit_id = gh_repo.refname_to_id("HEAD").unwrap();
            gh_repo.find_commit(commit_id).unwrap()
        };
        let signature = Signature::now(util::GIT_NAME, util::GIT_EMAIL).unwrap();
        gh_repo
            .commit(
                Some("HEAD"),
                &signature,
                &signature,
                "Update Prebuilt-MPR per-branch package files [ci skip]",
                &tree,
                &[&prev_commit],
            )
            .unwrap();
        gh_remote.push(&[gh_branch.name().unwrap()], None).unwrap();
    }
}
