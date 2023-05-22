use once_cell::sync::Lazy;
use reqwest::Client;

/// The URL of the MPR.
pub static MPR_URL: &str = "mpr.makedeb.org";

/// The GitHub organization that the Prebuilt-MPR system runs in.
pub static PBMPR_GITHUB_ORG: &str = "makedeb";

/// The GitHub repository that the Prebuilt-MPR system runs in.
pub static PBMPR_GITHUB_REPO: &str = "prebuilt-mpr-gh-actions";

/// A [`reqwest::Client`] that can be used across this crate. This global is used in case any
/// future requirements make it to where all requests need to have certain options applied to them.
pub static CLIENT: Lazy<Client> = Lazy::new(Client::new);
