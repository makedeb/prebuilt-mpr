use crate::{pkglist, util};
use serde_json::json;
use std::{thread, time::Duration};

pub async fn run_checks() -> exitcode::ExitCode {
    let crab = octocrab::instance();
    let actions = crab.actions();
    let _count = 0;

    for pkgbase in pkglist::PKGLIST.keys() {
        // Run the workflow job.
        log::info!("Creating update check job for '{pkgbase}'...");
        actions
            .create_workflow_dispatch(
                util::PBMPR_GITHUB_ORG,
                util::PBMPR_GITHUB_REPO,
                "check-pkg.yml",
                "main",
            )
            .inputs(json!({ "pkgbase": pkgbase }))
            .send()
            .await
            .unwrap();

        // Wait a little bit so we don't hit the 500 runs/10s limit on GitHub.
        thread::sleep(Duration::from_millis(500));
    }

    exitcode::OK
}
