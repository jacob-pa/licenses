use crate::filter::Filter;
use anyhow::Context;
pub use cargo_metadata::Metadata;
use serde::{Deserialize, Deserializer};
use std::path::Path;
use std::str::FromStr;

pub fn crate_metadata(project_directory: &Path) -> anyhow::Result<Metadata> {
    cargo_metadata::MetadataCommand::new()
        .current_dir(project_directory)
        .exec()
        .context("failed to execute cargo metadata")
}

pub fn config(metadata: &Metadata) -> anyhow::Result<Config> {
    let package = metadata
        .packages
        .iter()
        .find(|p| p.id == metadata.workspace_members[0])
        .expect("malformed metadata");
    match package.metadata.get("licenses") {
        Some(value) => serde_json::from_value::<Config>(value.clone())
            .context("failed to parse lint rules from [package.metadata.licenses]"),
        None => Ok(Config::default()),
    }
}

#[derive(Deserialize, Default)]
pub struct Config {
    #[serde(default, deserialize_with = "filters_from_strings")]
    pub allow: Vec<Filter>,
    #[serde(default, deserialize_with = "filters_from_strings")]
    pub warn: Vec<Filter>,
    #[serde(default, deserialize_with = "filters_from_strings")]
    pub deny: Vec<Filter>,
}

fn filters_from_strings<'de, D>(deserializer: D) -> Result<Vec<Filter>, D::Error>
where
    D: Deserializer<'de>,
{
    Vec::<String>::deserialize(deserializer)?
        .into_iter()
        .map(|s| Filter::from_str(&s).map_err(serde::de::Error::custom))
        .collect()
}
