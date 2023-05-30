use crate::util;
use flate2::read::GzDecoder;
use reqwest::StatusCode;
use serde::Deserialize;
use std::io::prelude::*;

#[derive(Deserialize)]
pub struct MprPackage {
    #[serde(rename = "PackageBase")]
    pub pkgbase: String,
    #[serde(rename = "Version")]
    pub version: String,
}

pub async fn get_mpr_packages() -> anyhow::Result<Vec<MprPackage>> {
    let archive_url = format!("https://{}/packages-meta-ext-v2.json.gz", util::MPR_URL);
    let resp = util::HTTP_CLIENT.get(archive_url).send().await.unwrap();

    let status = resp.status();
    anyhow::ensure!(status == StatusCode::OK, "Invalid response code ({status})");

    let bytes = resp.bytes().await.unwrap();
    let mut decoder = GzDecoder::new(bytes.as_ref());
    let mut json_packages = String::new();
    decoder.read_to_string(&mut json_packages).unwrap();

    Ok(serde_json::from_str(&json_packages).unwrap())
}
