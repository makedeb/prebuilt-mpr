use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;

/// The package list from the 'packages.toml' file.
pub static PKGLIST: Lazy<HashMap<String, Package>> =
    Lazy::new(|| toml::de::from_str(include_str!("../packages.toml")).unwrap());

#[derive(Deserialize)]
pub struct Package {
    #[serde(default)]
    pub blocked_distros: Vec<BlockedDistros>,
}

#[derive(Deserialize)]
pub enum BlockedDistros {
    #[serde(rename = "focal")]
    Focal,
    #[serde(rename = "bullseye")]
    Bullseye,
}
