use crate::Arguments;
use anyhow::Context;
use cargo_metadata::{DepKindInfo, DependencyKind, Metadata, PackageId};
use std::path::PathBuf;

pub struct Package {
    pub name: String,
    pub repository: Option<String>,
    pub project_folder: PathBuf,
    pub spdx_license: Option<spdx::Expression>,
}

impl From<cargo_metadata::Package> for Package {
    fn from(package: cargo_metadata::Package) -> Self {
        Self {
            name: package.name.replace('-', "_"),
            repository: package.repository,
            project_folder: package
                .manifest_path
                .as_std_path()
                .parent()
                .expect("manifest not in a folder")
                .to_path_buf(),
            spdx_license: package
                .license
                .and_then(|l| spdx::Expression::parse(&l).ok()),
        }
    }
}

pub fn dependencies(args: &Arguments) -> anyhow::Result<impl Iterator<Item = Package>> {
    let metadata = cargo_metadata::MetadataCommand::new()
        .current_dir(&args.project_directory)
        .exec()
        .context("failed to execute cargo metadata")?;
    let included = included_ids(
        &metadata,
        &excluded_ids(&metadata, &args.excluded),
        args.build_dependencies,
        args.dev_dependencies,
    );
    Ok(metadata
        .packages
        .into_iter()
        .filter(move |package| included.contains(&package.id))
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

fn included_ids(
    metadata: &Metadata,
    excluded: &[&PackageId],
    build_dependencies: bool,
    dev_dependencies: bool,
) -> Vec<PackageId> {
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
                .deps
                .iter()
                .filter(|dep| is_included(&dep.dep_kinds, build_dependencies, dev_dependencies))
                .map(|dep| &dep.pkg)
                .filter(|id| !excluded.contains(id))
                .filter(|id| !included.contains(*id)),
        );
        if !excluded.contains(&package_id) && !metadata.workspace_members.contains(package_id) {
            included.push(package_id.to_owned())
        }
    }
    included
}

fn is_included(
    kind_info: &[DepKindInfo],
    build_dependencies: bool,
    dev_dependencies: bool,
) -> bool {
    kind_info.iter().map(|k| k.kind).any(|kind| match kind {
        DependencyKind::Build => build_dependencies,
        DependencyKind::Development => dev_dependencies,
        _ => true,
    })
}
