use anyhow::Context;
use serde::Deserialize;
use std::path::Path;

use crate::lint::Lint;
pub use cargo_metadata::Metadata;

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
    pub allow: Vec<Lint>,
}
