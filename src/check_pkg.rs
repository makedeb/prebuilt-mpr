use crate::{
    cache::{self, MprPackage},
    util,
};
use git2::{BranchType, IndexAddOption, ObjectType, Remote, Repository, Signature};
use rust_apt::{cache::Cache, util as apt_util};
use std::{
    cmp::Ordering,
    fs::{self, File},
    io::prelude::*,
    path::Path,
};

/// The GitHub Actions package updater workflow file. See the code usages in
/// this file for more info.
static PKG_UPDATE_ACTION: &[u8] = include_bytes!("actions/update-pkg.yml");

/// The GitHub Actions package publisher workflow file. See the code usages in
/// this file for more info.
static PKG_PUBLISH_ACTION: &[u8] = include_bytes!("actions/publish-pkg.yml");

/// The CODEOWNERS file we use to review the proper people on package updates.
static PKG_CODEOWNERS: &[u8] = include_bytes!("actions/CODEOWNERS");

pub async fn check_pkg(gh_user: &str, gh_token: &str, pkg: &str) -> exitcode::ExitCode {
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

    let cache = Cache::new::<&str>(&[]).unwrap();
    let apt_pkg = cache.get(pkg);

    // If the Prebuilt-MPR version is less than that on the MPR (or the Prebuilt-MPR
    // package just doesn't exist yet), than the Prebuilt-MPR package needs to
    // be updated to match that on the MPR.
    if apt_pkg.is_some()
        && apt_util::cmp_versions(
            apt_pkg.unwrap().candidate().unwrap().version(),
            &package.version,
        ) == Ordering::Less
    {
        log::info!("Updating '{pkg}'...");
        update_pkg(gh_user, gh_token, package).await;
    } else {
        log::info!("'{pkg}' is up to date!");
    };

    exitcode::OK
}

async fn update_pkg(gh_user: &str, gh_token: &str, pkg: &MprPackage) {
    // Stuff we need throughout this function.
    let mpr_repo_url = format!("https://{}/{}", util::MPR_URL, pkg.pkgbase);
    let gh_repo_url = format!(
        "https://{gh_user}:{gh_token}@github.com/{}/{}",
        util::PBMPR_GITHUB_ORG,
        util::PBMPR_GITHUB_REPO
    );
    let gh_pkg_branch = format!("pkg/{}", pkg.pkgbase);
    let gh_pkg_update_branch = format!("pkg-update/{}", pkg.pkgbase);

    // Set up the octocrab instance.
    let crab = octocrab::instance();
    let pulls = crab.pulls(util::PBMPR_GITHUB_ORG, util::PBMPR_GITHUB_REPO);

    let active_pulls = async || {
        pulls
            .list()
            .head(&gh_pkg_update_branch)
            .base(&gh_pkg_branch)
            .send()
            .await
            .unwrap()
            .items
    };

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

        // If the branch existed on the remote, configure the local branch to point to the remote
        // branch.
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
    for maybe_file in fs::read_dir("mpr-repo/").unwrap() {
        let file = maybe_file.unwrap();
        let file_name = file.file_name().into_string().unwrap();

        if [".git", ".github", ".SRCINFO"].contains(&file_name.as_str()) {
            continue;
        }

        fs::copy(
            file.path(),
            format!("gh-repo/{}", file.file_name().into_string().unwrap()),
        )
        .unwrap();
    }

    // Add the new files into the GitHub branch.
    let mut gh_index = gh_repo.index().unwrap();
    gh_index
        .add_all(["*", ".*"], IndexAddOption::DEFAULT, None)
        .unwrap();
    gh_index.write().unwrap();

    // Commit and push the new files into the the GitHub branch, if anything was
    // changed.
    if gh_repo.statuses(None).unwrap().is_empty() {
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

    // Set up the PR to merge in the changes, if no existing PR is open.
    if !active_pulls().await.is_empty() {
        log::info!("PR already exists, skipping PR creation.");
    } else {
        log::info!("Creating PR...");
        pulls
            .create(
                format!("Update `{}` to `{}`", pkg.pkgbase, pkg.version),
                &gh_pkg_update_branch,
                &gh_pkg_branch,
            )
            .maintainer_can_modify(true)
            .send()
            .await
            .unwrap();
    }
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
    File::create("gh-repo/.github/workflows/publish-pkg.yml")
        .unwrap()
        .write_all(PKG_PUBLISH_ACTION)
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
                [
                    ".github/workflows/update-pkg.yml",
                    ".github/workflows/publish-pkg.yml",
                    ".github/CODEOWNERS",
                ],
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
