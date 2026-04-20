use crate::Lint;
use crate::identity::IdentifiedLicense;
use crate::lint::{Level, Report};
use crate::package::Package;
use spdx::{LicenseId, LicenseItem, Licensee};

pub fn extraneous(
    dependencies: &[Package],
    licenses: &[IdentifiedLicense],
) -> impl Iterator<Item = Report> {
    dependencies
        .iter()
        .filter_map(|package| {
            package
                .spdx_license
                .as_ref()
                .map(|expression| (package, expression))
        })
        .flat_map(|(package, expression)| {
            extraneous_package_licenses(package, expression, licenses)
        })
        .map(|item| Report {
            lint: Lint::Extraneous,
            level: Level::Info,
            item,
        })
}

fn extraneous_package_licenses<'a>(
    package: &Package,
    expression: &spdx::Expression,
    licenses: &[IdentifiedLicense<'a>],
) -> Vec<String> {
    let package_licenses: Vec<_> = licenses
        .iter()
        .filter(|l| l.license.package_id == package.id)
        .collect();
    match minimal_requirements(expression, &package_licenses) {
        Some(required) => package_licenses
            .into_iter()
            .filter(|l| !required.iter().any(|r| r.name != l.license.name))
            .map(|l| format!("{} (not {})", l.license.location_file_name(), expression))
            .collect(),
        None => Vec::new(),
    }
}

fn minimal_requirements<'a>(
    expression: &spdx::Expression,
    licenses: &[&IdentifiedLicense<'a>],
) -> Option<Vec<LicenseId>> {
    let licensee: Vec<_> = licenses
        .iter()
        .flat_map(|l| l.ids())
        .map(|id| {
            Licensee::new(
                LicenseItem::Spdx {
                    id: *id,
                    or_later: false,
                },
                None,
            )
        })
        .collect();
    match expression.minimized_requirements(licensee.iter()) {
        Ok(requirements) => Some(requirements.iter().filter_map(|l| l.license.id()).collect()),
        Err(_) => None,
    }
}
