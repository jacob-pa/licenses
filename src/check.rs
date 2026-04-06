use crate::local::Local;
use crate::package::Package;
use crate::{Arguments, local, package};
use spdx::LicenseId;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub fn check(args: &Arguments) -> anyhow::Result<()> {
    let dependencies: Vec<_> =
        package::dependencies(&args.project_directory, &args.excluded)?.collect();
    let licenses = local::output_folder_licenses(&args.output_directory);
    let (missing, unexpected) = missing_or_unexpected_licenses(&dependencies, &licenses);

    if !missing.is_empty() {
        println!(
            "{} dependencies missing licenses: {}",
            missing.len(),
            missing.join(", ")
        );
    }

    if !unexpected.is_empty() {
        println!(
            "{} unused licenses found in output folder: {}",
            unexpected.len(),
            unexpected.join(", ")
        );
    }

    let unknown: Vec<_> = licenses
        .iter()
        .map(IdentifiedLicense::from)
        .filter(|l| l.id_from_name.is_none())
        .map(|l| l.license.name.clone())
        .collect();
    println!(
        "{} unknown license types out of {}: {}",
        unknown.len(),
        licenses.len(),
        unknown.join(", ")
    );
    Ok(())
}

fn missing_or_unexpected_licenses(
    dependencies: &[Package],
    licenses: &[Local],
) -> (Vec<String>, Vec<String>) {
    let expected: HashSet<_> = dependencies.iter().map(|p| p.name.clone()).collect();
    let found: HashSet<_> = licenses.iter().map(|l| l.package.clone()).collect();
    let mut missing: Vec<_> = expected.difference(&found).cloned().collect();
    let mut unexpected: Vec<_> = found.difference(&expected).cloned().collect();
    missing.sort();
    unexpected.sort();
    (missing, unexpected)
}

struct IdentifiedLicense<'a> {
    license: &'a Local,
    id_from_name: Option<LicenseId>,
}

impl<'a> From<&'a Local> for IdentifiedLicense<'a> {
    fn from(license: &'a Local) -> Self {
        Self {
            id_from_name: id_from_name(&license.location),
            license,
        }
    }
}

fn id_from_name(path: &Path) -> Option<LicenseId> {
    // slightly arbitrarily preferring earlier words, and more precise names
    path.file_name()?
        .to_str()?
        .split('-')
        .skip(1) // package name
        .flat_map(possible_ids_from_word)
        .next()
}

fn possible_ids_from_word(word: &str) -> impl Iterator<Item = LicenseId> {
    let precise = spdx::license_id(word).into_iter();
    let imprecise = spdx::imprecise_license_id(word).map(|(id, _)| id);
    precise.chain(imprecise)
}
