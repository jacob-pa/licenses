use anyhow::Context;
use indicatif::ProgressIterator;
use spdx::{LicenseId, LicenseItem, LicenseReq, Licensee};

use crate::PruneArguments;
use crate::identity::IdentifiedLicense;
use crate::package::Package;
use std::path::PathBuf;
use std::process::ExitCode;

pub fn prune(args: &PruneArguments) -> anyhow::Result<ExitCode> {
    let reporter = crate::reporter::Reporter::new(args.common.quiet);
    let metadata = crate::metadata::crate_metadata(&args.common.project_directory)?;
    let dependencies: Vec<_> = crate::package::dependencies(&args.common, &metadata).collect();
    let licenses = crate::local::output_folder_licenses(&args.common.license_directory);
    let licenses = crate::identity::identified_licenses(&licenses)?;
    let extraneous: Vec<_> = extraneous_licenses(&args.licenses, &dependencies, &licenses);

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
    let requirements: Vec<_> = dependencies
        .iter()
        .flat_map(|package| package_requirements(preference, package, licenses))
        .collect();
    licenses
        .iter()
        .filter(|l| !requirements.iter().any(|r| r == &l.license.location))
        .map(|l| l.license.location.clone())
        .collect()
}

fn package_requirements(
    preference: &[spdx::Licensee],
    package: &Package,
    licenses: &[IdentifiedLicense],
) -> Vec<PathBuf> {
    let package_licenses: Vec<_> = licenses
        .iter()
        .filter(|l| l.license.package == package.name)
        .collect();
    package
        .spdx_license
        .as_ref()
        .and_then(|expression| minimal_requirements(preference, expression, &package_licenses))
        .unwrap_or_else(|| {
            package_licenses
                .into_iter()
                .map(|l| l.license.location.clone())
                .collect()
        })
}

fn minimal_requirements(
    preference: &[spdx::Licensee],
    expression: &spdx::Expression,
    licenses: &[&IdentifiedLicense],
) -> Option<Vec<PathBuf>> {
    let licensees: Vec<_> = sort_requirements(
        preference,
        licenses
            .iter()
            .flat_map(|l| l.ids())
            .map(licensee_from_id)
            .collect(),
    );
    let requirements = match expression.minimized_requirements(&licensees) {
        Ok(requirements) => requirements,
        Err(_) => return None,
    };
    Some(
        licenses
            .iter()
            .filter(|l| required_license(&requirements, l))
            .map(|l| l.license.location.clone())
            .collect(),
    )
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
