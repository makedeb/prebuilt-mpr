use git2::{BranchType, Repository, Signature};
use once_cell::sync::Lazy;
use reqwest::Client;

/// The URL of the MPR.
pub static MPR_URL: &str = "mpr.makedeb.org";

/// The GitHub organization that the Prebuilt-MPR system runs in.
pub static PBMPR_GITHUB_ORG: &str = "makedeb";

/// The GitHub repository that the Prebuilt-MPR system runs in.
pub static PBMPR_GITHUB_REPO: &str = "prebuilt-mpr-gh-actions";

/// Our Git name.
pub static GIT_NAME: &str = "Kavplex Bot";

/// Our Git email.
pub static GIT_EMAIL: &str = "kavplex@hunterwittenborn.com";

/// A [`reqwest::Client`] that can be used across this crate. This global is used in case any
/// future requirements make it to where all requests need to have certain options applied to them.
pub static CLIENT: Lazy<Client> = Lazy::new(Client::new);

/// Get our Git signature.
pub fn git_signature() -> Signature<'static> {
    Signature::now(GIT_NAME, GIT_EMAIL).unwrap()
}

/// Get the list of branches in a given repository.
pub fn get_branch_names(repo: &Repository, branch_type: BranchType) -> Vec<String> {
    repo.branches(Some(branch_type))
        .unwrap()
        .map(|branch_tuple| {
            let mut branch = branch_tuple.unwrap().0.name().unwrap().unwrap().to_owned();
            if branch.starts_with("origin/") {
                branch = branch.strip_prefix("origin/").unwrap().to_owned();
            }
            branch
        })
        .filter(|branch| branch != "HEAD")
        .collect()
}
