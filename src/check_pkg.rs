use crate::util;
use flate2::read::GzDecoder;
use reqwest::StatusCode;
use rust_apt::{cache::Cache, util as apt_util};
use serde::Deserialize;
use std::{cmp::Ordering, io::prelude::*};

#[derive(Deserialize)]
struct Package {
    #[serde(rename = "PackageBase")]
    package_base: String,
    #[serde(rename = "Version")]
    version: String,
}

pub async fn check_pkg(pkg: String) -> exitcode::ExitCode {
    let archive_url = format!("https://{}/packages-meta-ext-v2.json.gz", util::MPR_URL);

    let resp = util::CLIENT.get(archive_url).send().await.unwrap();

    let status = resp.status();
    if status != StatusCode::OK {
        log::error!("Failed to fetch archive for package metadata from the MPR: {status}.");
        return exitcode::TEMPFAIL;
    };

    let bytes = resp.bytes().await.unwrap();
    let mut decoder = GzDecoder::new(bytes.as_ref());
    let mut json_packages = String::new();
    decoder.read_to_string(&mut json_packages).unwrap();

    let packages: Vec<Package> = serde_json::from_str(&json_packages).unwrap();
    let package = match packages
        .iter()
        .find(|json_pkg| json_pkg.package_base == pkg)
    {
        Some(package) => package,
        None => {
            log::error!("The package '{pkg}' couldn't be found on the MPR.");
            return exitcode::DATAERR;
        }
    };

    let cache = Cache::new::<&str>(&[]).unwrap();
    let apt_pkg = match cache.get(&pkg) {
        Some(apt_pkg) => apt_pkg,
        None => {
            log::error!("The package '{pkg}' couldn't be found in the APT cache.");
            return exitcode::OSERR;
        }
    };

    // If the Prebuilt-MPR version is less than that on the MPR, than the Prebuilt-MPR package
    // needs to be updated to match that on the MPR.
    if apt_util::cmp_versions(apt_pkg.candidate().unwrap().version(), &package.version)
        == Ordering::Less
    {
        log::info!("Package needs updated!");
    } else {
        log::info!("Package is up to date!");
    };

    exitcode::OK
}
