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
#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::package::Version;
    use cargo_metadata::camino::Utf8Path;
    use cargo_metadata::{PackageBuilder, PackageName};

    #[derive(Default)]
    pub struct FakeMetadata {
        workspace_members: Vec<PackageId>,
        packages: Vec<Package>,
    }

    impl FakeMetadata {
        pub fn workspace_member(&mut self, name: &str, version: &str) -> PackageId {
            let name = PackageName::new(name.to_string());
            let version = Version::parse(version).unwrap();
            let id = PackageId {
                repr: format!("{}-{}", name.replace('-', "_"), version),
            };
            let path_stub = Utf8Path::new(&id.repr);
            let package = PackageBuilder::new(name, version, id.clone(), path_stub)
                .build()
                .unwrap();
            self.workspace_members.push(id.clone());
            self.packages.push(package);
            id
        }
    }

    impl Metadata for FakeMetadata {
        fn workspace_members(&self) -> &Vec<PackageId> {
            &self.workspace_members
        }

        fn resolve(&self) -> &Option<Resolve> {
            todo!()
        }

        fn packages(&self) -> &Vec<Package> {
            &self.packages
        }
    }
}
