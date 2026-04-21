use crate::metadata::Metadata;
use crate::{config::SearchRemote, filter::Filter};
use anyhow::Context;
use serde::Deserialize;
use std::path::PathBuf;

pub fn parse_metadata_toml(metadata: &impl Metadata) -> anyhow::Result<TomlConfig> {
    let package = metadata
        .packages()
        .iter()
        .find(|p| p.id == metadata.workspace_members()[0])
        .expect("malformed metadata");
    let empty = serde_json::Value::Object(serde_json::Map::new());
    let value = package.metadata.get("licenses").unwrap_or(&empty);
    serde_json::from_value::<TomlConfig>(value.clone())
        .context("failed to parse lint rules from [package.metadata.licenses]")
}

#[derive(serde::Deserialize)]
pub struct TomlConfig {
    #[serde(flatten)]
    pub common: TomlCommon,
    pub search_remote: Option<SearchRemote>,
    pub keywords: Option<Vec<String>>,
    #[serde(default, deserialize_with = "vec_from_strings")]
    pub allow: Option<Vec<Filter>>,
    #[serde(default, deserialize_with = "vec_from_strings")]
    pub warn: Option<Vec<Filter>>,
    #[serde(default, deserialize_with = "vec_from_strings")]
    pub deny: Option<Vec<Filter>>,
    #[serde(default, deserialize_with = "vec_from_strings")]
    pub licenses: Option<Vec<spdx::Licensee>>,
}

#[derive(serde::Deserialize)]
pub struct TomlCommon {
    pub license_directory: Option<PathBuf>,
    pub excluded: Option<Vec<String>>,
    pub build_dependencies: Option<bool>,
    pub dev_dependencies: Option<bool>,
    pub quiet: Option<bool>,
}

fn vec_from_strings<'de, D, S>(deserializer: D) -> Result<Option<Vec<S>>, D::Error>
where
    D: serde::Deserializer<'de>,
    S: std::str::FromStr,
    S::Err: std::fmt::Display,
{
    let strings = match Option::<Vec<String>>::deserialize(deserializer)? {
        Some(strings) => strings,
        None => return Ok(None),
    };
    Ok(Some(
        strings
            .into_iter()
            .map(|s: String| S::from_str(&s).map_err(serde::de::Error::custom))
            .collect::<Result<Vec<_>, _>>()?,
    ))
}
