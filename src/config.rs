use crate::{CommonConfig, FilterConfig};
use anyhow::Context;
pub use cargo_metadata::Metadata;
use serde::Deserialize;
use std::path::Path;

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
    let empty = serde_json::Value::Object(serde_json::Map::new());
    let value = package.metadata.get("licenses").unwrap_or(&empty);
    serde_json::from_value::<Config>(value.clone())
        .context("failed to parse lint rules from [package.metadata.licenses]")
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(flatten)]
    pub common: CommonConfig,

    #[serde(flatten)]
    pub filter: FilterConfig,
}
