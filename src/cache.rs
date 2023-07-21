use crate::util;
use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MprPackage {
    #[serde(rename = "PackageBase")]
    pub pkgbase: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Version")]
    pub version: String,
}

pub async fn get_mpr_packages() -> anyhow::Result<Vec<MprPackage>> {
    let archive_url = format!("https://{}/packages-meta-ext-v2.json.gz", util::MPR_URL);
    let resp = util::HTTP_CLIENT.get(archive_url).send().await.unwrap();

    let status = resp.status();
    anyhow::ensure!(status == StatusCode::OK, "Invalid response code ({status})");
    let packages = String::from_utf8(resp.bytes().await.unwrap().to_vec()).unwrap();

    Ok(serde_json::from_str(&packages).unwrap())
}
