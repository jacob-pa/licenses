use crate::Arguments;
use crate::identity::IdentifiedLicense;
use crate::local::Local;
use crate::package::Package;
use spdx::LicenseId;
use std::collections::HashSet;

pub fn check(args: &Arguments) -> anyhow::Result<()> {
    let dependencies: Vec<_> =
        crate::package::dependencies(&args.project_directory, &args.excluded)?.collect();
    let licenses = crate::local::output_folder_licenses(&args.output_directory);
    let (missing, unexpected) = missing_or_unexpected_licenses(&dependencies, &licenses);
    let licenses = crate::identity::identified_licenses(&licenses)?;
    let unknown = unknown_license_types(&licenses);
    let copy_left = copy_left_licenses(&licenses);

    if !missing.is_empty() {
        println!(
            "{} dependencies missing licenses: {}",
            missing.len(),
            missing.join(", ")
        );
    }

    if !unexpected.is_empty() {
        println!(
            "{} unused dependency licenses found in output folder: {}",
            unexpected.len(),
            unexpected.join(", ")
        );
    }

    if !unknown.is_empty() {
        println!(
            "{} unknown license types: {}",
            unknown.len(),
            unknown.join(", ")
        );
    }

    if !copy_left.is_empty() {
        println!(
            "{} maybe copy-left licenses found: {}",
            copy_left.len(),
            copy_left.join(", ")
        );
    }

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

fn unknown_license_types(licenses: &[IdentifiedLicense]) -> Vec<String> {
    licenses
        .iter()
        .filter(|l| l.id().is_none())
        .map(|l| l.license.location.to_string_lossy().to_string())
        .collect()
}

fn copy_left_licenses(licenses: &[IdentifiedLicense]) -> Vec<String> {
    licenses
        .iter()
        .filter(|l| l.id().map(LicenseId::is_copyleft).unwrap_or(false))
        .map(|l| l.license.location.to_string_lossy().to_string())
        .collect()
}
