use anyhow::Context;
use cargo_metadata::semver::Version;
use cargo_metadata::{Metadata, PackageId};
use std::path::{Path, PathBuf};

pub struct Package {
    pub name: String,
    pub version: Version,
    pub license: Option<String>,
    pub project_folder: PathBuf,
}

impl From<cargo_metadata::Package> for Package {
    fn from(package: cargo_metadata::Package) -> Self {
        Self {
            name: package.name.to_string(),
            version: package.version,
            license: package.license,
            project_folder: package
                .manifest_path
                .as_std_path()
                .parent()
                .expect("manifest not in a folder")
                .to_path_buf(),
        }
    }
}

pub fn dependencies(
    project_path: &Path,
    excluded: &[String],
) -> anyhow::Result<impl Iterator<Item = Package>> {
    let metadata = cargo_metadata::MetadataCommand::new()
        .current_dir(project_path)
        .exec()
        .context("failed to execute cargo metadata")?;
    let included = included_ids(&metadata, &excluded_ids(&metadata, excluded));
    Ok(metadata
        .packages
        .into_iter()
        .filter(move |package| included.contains(&&package.id))
        .map(Package::from))
}

fn excluded_ids<'m>(metadata: &'m Metadata, excluded: &[String]) -> Vec<&'m PackageId> {
    metadata
        .packages
        .iter()
        .filter(|package| excluded.contains(&package.name))
        .map(|package| &package.id)
        .collect()
}

fn included_ids(metadata: &Metadata, excluded: &[&PackageId]) -> Vec<PackageId> {
    let mut unvisited: Vec<_> = metadata
        .workspace_members
        .iter()
        .filter(|member| !excluded.contains(member))
        .collect();
    let mut included = Vec::new();
    let nodes = &metadata.resolve.as_ref().unwrap().nodes;
    while let Some(package_id) = unvisited.pop() {
        unvisited.extend(
            nodes
                .iter()
                .find(|package| package.id == *package_id)
                .unwrap()
                .dependencies
                .iter()
                .filter(|id| !excluded.contains(id))
                .filter(|id| !included.contains(*id)),
        );
        if !excluded.contains(&package_id) && !metadata.workspace_members.contains(&package_id) {
            included.push(package_id.to_owned())
        }
    }
    included
}
