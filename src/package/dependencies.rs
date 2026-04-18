use super::Package;
use crate::Arguments;
use cargo_metadata::{DepKindInfo, DependencyKind, Metadata};
use itertools::Itertools;

pub fn root_package(metadata: &Metadata) -> Package {
    metadata
        .packages
        .iter()
        .find(|p| p.id == metadata.workspace_members[0])
        .expect("malformed metadata")
        .into()
}

pub fn dependencies(args: &Arguments, metadata: &Metadata) -> impl Iterator<Item = Package> {
    let included = included_ids(
        metadata,
        &excluded_ids(metadata, &args.excluded),
        args.build_dependencies,
        args.dev_dependencies,
    );
    metadata
        .packages
        .iter()
        .filter(move |package| included.contains(&package.id))
        .map(Package::from)
        .dedup_by(|a, b| a.id == b.id)
}

fn excluded_ids<'m>(
    metadata: &'m Metadata,
    excluded: &[String],
) -> Vec<&'m cargo_metadata::PackageId> {
    metadata
        .packages
        .iter()
        .filter(|package| excluded.contains(&package.name))
        .map(|package| &package.id)
        .collect()
}

fn included_ids(
    metadata: &Metadata,
    excluded: &[&cargo_metadata::PackageId],
    build_dependencies: bool,
    dev_dependencies: bool,
) -> Vec<cargo_metadata::PackageId> {
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
