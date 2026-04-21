use anyhow::Context;
use std::path::Path;

pub use cargo_metadata::{DepKindInfo, DependencyKind, Package, PackageId, Resolve};

pub fn crate_metadata(project_directory: &Path) -> anyhow::Result<impl Metadata + use<>> {
    cargo_metadata::MetadataCommand::new()
        .current_dir(project_directory)
        .exec()
        .context("failed to execute cargo metadata")
}

pub trait Metadata {
    fn workspace_members(&self) -> &Vec<PackageId>;
    fn resolve(&self) -> &Option<Resolve>;
    fn packages(&self) -> &Vec<Package>;
}

impl Metadata for cargo_metadata::Metadata {
    fn workspace_members(&self) -> &Vec<PackageId> {
        &self.workspace_members
    }

    fn resolve(&self) -> &Option<Resolve> {
        &self.resolve
    }

    fn packages(&self) -> &Vec<Package> {
        &self.packages
    }
}
