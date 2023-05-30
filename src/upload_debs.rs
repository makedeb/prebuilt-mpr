use crate::util;
use octocrab::params::actions::ArchiveFormat;
use proget::Client;
use std::io::{prelude::*, Cursor};
use zip::ZipArchive;

struct DebPkg {
    file_name: String,
    file_content: Vec<u8>,
    distro: String,
}

pub async fn upload_debs(pr_num: u64, proget_token: &str) -> exitcode::ExitCode {
    let crab = octocrab::instance();
    let pulls = crab.pulls(util::PBMPR_GITHUB_ORG, util::PBMPR_GITHUB_REPO);
    let workflows = crab.workflows(util::PBMPR_GITHUB_ORG, util::PBMPR_GITHUB_REPO);
    let actions = crab.actions();

    log::info!("Looking up PR...");
    let pr = match pulls.get(pr_num).await {
        Ok(pr) => pr,
        Err(err) => {
            log::error!("Failed to find PR ({err})");
            return exitcode::TEMPFAIL;
        }
    };

    log::info!("Looking up latest run...");
    let run = {
        let runs = workflows
            .list_runs("update-pkg.yml")
            .branch(&pr.head.ref_field)
            .event("pull_request")
            .send()
            .await
            .unwrap()
            .take_items();

        let mut valid_runs = runs
            .into_iter()
            .filter(|run| run.head_sha == pr.head.sha)
            .collect::<Vec<_>>();

        assert_eq!(valid_runs.len(), 1);
        valid_runs.pop().unwrap()
    };

    log::info!("Looking up artifacts...");
    let artifacts = actions
        .list_workflow_run_artifacts(util::PBMPR_GITHUB_ORG, util::PBMPR_GITHUB_REPO, run.id)
        .send()
        .await
        .unwrap()
        .value
        .unwrap()
        .take_items();

    let mut debs: Vec<DebPkg> = vec![];

    for artifact in artifacts {
        log::info!("Downloading artifacts for '{}'...", artifact.name);

        let bytes = actions
            .download_artifact(
                util::PBMPR_GITHUB_ORG,
                util::PBMPR_GITHUB_REPO,
                artifact.id,
                ArchiveFormat::Zip,
            )
            .await
            .unwrap();
        let cursor = Cursor::new(bytes);
        let mut zip_archive = ZipArchive::new(cursor).unwrap();

        for index in 0..zip_archive.len() {
            let mut file = zip_archive.by_index(index).unwrap();
            let file_name = file.enclosed_name().unwrap().to_str().unwrap().to_owned();
            let mut file_content = vec![];
            file.read_to_end(&mut file_content).unwrap();

            // The distribution codename, with the 'debian-' or 'ubuntu-' prefix removed
            // from it, as done in 'src/actions/update-pkg.yml'.
            let distro = if artifact.name.starts_with("debian-") {
                artifact.name.strip_prefix("debian-").unwrap()
            } else {
                artifact.name.strip_prefix("ubuntu-").unwrap()
            }
            .to_owned();

            debs.push(DebPkg {
                file_name,
                file_content,
                distro,
            });
        }
    }

    let pg_url = format!("https://{}", util::PROGET_URL);
    let pg = Client::new(pg_url.parse().unwrap(), proget_token);
    for deb in &debs {
        log::info!(
            "Uploading '{}' to '{}' on ProGet...",
            deb.file_name,
            deb.distro
        );
        pg.upload_deb(
            "prebuilt-mpr",
            &deb.distro,
            &deb.file_name,
            &deb.file_content,
        )
        .await
        .unwrap()
    }

    exitcode::OK
}
