use anyhow::Context;
use indicatif::ProgressIterator;
use spdx::{LicenseId, LicenseItem, LicenseReq, Licensee};

use crate::config::PruneConfig;
use crate::identity::IdentifiedLicense;
use crate::metadata::Metadata;
use crate::package::Package;
use crate::reporter::Reporter;
use std::path::PathBuf;
use std::process::ExitCode;

pub fn prune(
    metadata: impl Metadata,
    config: PruneConfig,
    reporter: Reporter,
) -> anyhow::Result<ExitCode> {
    let dependencies: Vec<_> = crate::package::dependencies(&config.common, &metadata).collect();
    let licenses = crate::license::output_folder_licenses(&config.common.license_directory);
    let licenses = crate::identity::identified_licenses(&licenses)?;
    let extraneous = extraneous_licenses(&config.licenses, &dependencies, &licenses);

    reporter.info(format!(
        "removing {} extraneous license(s)",
        extraneous.len()
    ));

    for license in extraneous.iter().progress_count(extraneous.len() as u64) {
        std::fs::remove_file(license)
            .with_context(|| format!("failed to remove {}", license.display()))?;
    }

    Ok(reporter.exit_code())
}

fn extraneous_licenses(
    preference: &[spdx::Licensee],
    dependencies: &[Package],
    licenses: &[IdentifiedLicense],
) -> Vec<PathBuf> {
    dependencies
        .iter()
        .flat_map(|package| extraneous_requirements(preference, package, licenses))
        .collect()
}

fn extraneous_requirements(
    preference: &[spdx::Licensee],
    package: &Package,
    licenses: &[IdentifiedLicense],
) -> Vec<PathBuf> {
    let expression = match package.spdx_license.as_ref() {
        Some(expression) => expression,
        None => return Vec::new(),
    };
    let package_licenses: Vec<_> = licenses
        .iter()
        .filter(|l| l.license.package_id == package.id)
        .collect();
    let licensees: Vec<_> = sort_requirements(
        preference,
        package_licenses
            .iter()
            .flat_map(|l| l.ids())
            .map(licensee_from_id)
            .collect(),
    );
    let requirements = match expression.minimized_requirements(&licensees) {
        Ok(requirements) => requirements,
        Err(_) => return Vec::new(),
    };

    package_licenses
        .iter()
        .filter(|l| !l.ids_from_content.is_empty())
        .filter(|l| !required_license(&requirements, l))
        .map(|l| l.license.location.clone())
        .collect()
}

fn sort_requirements(
    preference: &[spdx::Licensee],
    mut requirements: Vec<spdx::Licensee>,
) -> Vec<spdx::Licensee> {
    requirements.sort_by(|item_a, item_b| {
        let pos_a = preference.iter().position(|x| x == item_a);
        let pos_b = preference.iter().position(|x| x == item_b);

        match (pos_a, pos_b) {
            (Some(a), Some(b)) => a.cmp(&b),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => item_a.cmp(item_b),
        }
    });
    requirements
}

fn required_license(requirements: &[LicenseReq], license: &IdentifiedLicense) -> bool {
    for requirement in requirements {
        for id in license.ids() {
            if requirement.license.id() == Some(*id) {
                return true;
            }
        }
    }
    false
}

fn licensee_from_id(id: &LicenseId) -> Licensee {
    Licensee::new(
        LicenseItem::Spdx {
            id: *id,
            or_later: false,
        },
        None,
    )
}
