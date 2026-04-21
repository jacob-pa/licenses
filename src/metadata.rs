use anyhow::Context;
use std::path::Path;

pub use cargo_metadata::Metadata;

pub fn crate_metadata(project_directory: &Path) -> anyhow::Result<Metadata> {
    cargo_metadata::MetadataCommand::new()
        .current_dir(project_directory)
        .exec()
        .context("failed to execute cargo metadata")
}
