use crate::Arguments;
use crate::identity::IdentifiedLicense;
use crate::local::Local;
use crate::package::Package;
use spdx::{LicenseId, LicenseItem, Licensee};
use std::collections::HashSet;
use std::path::PathBuf;
use std::process::ExitCode;

pub fn check(args: &Arguments) -> anyhow::Result<ExitCode> {
    let mut reporter = crate::report::Reporter::new(args);
    let dependencies: Vec<_> =
        crate::package::dependencies(&args.project_directory, &args.excluded)?.collect();
    let licenses = crate::local::output_folder_licenses(&args.output_directory);
    let (missing, unexpected) = missing_or_unexpected_licenses(&dependencies, &licenses);
    let licenses = crate::identity::identified_licenses(&licenses)?;
    let unknown = sorted(unknown_license_types(&licenses));
    let copy_left = sorted(copy_left_licenses(&licenses));
    let expressions = spdx_expressions(&dependencies)?;
    let unmet_spdx = sorted(packages_with_unmet_spdx(
        &dependencies,
        &expressions,
        &licenses,
    ));
    let extraneous = extraneous_licenses(&dependencies, &expressions, &licenses);

    if !missing.is_empty() {
        reporter.error(format!(
            "{} dependencies without any licenses: {}",
            missing.len(),
            missing.join(", ")
        ));
    }

    if !unmet_spdx.is_empty() {
        reporter.error(format!(
            "{} packages without licenses required by their Cargo.toml package.license field: {}",
            unmet_spdx.len(),
            unmet_spdx.join(", ")
        ));
    }

    if !copy_left.is_empty() {
        reporter.error(format!(
            "{} files with at least one copy-left license: {}",
            copy_left.len(),
            copy_left.join(", ")
        ));
    }

    if !unknown.is_empty() {
        reporter.warning(format!(
            "{} license files types with unknown types: {}",
            unknown.len(),
            unknown.join(", ")
        ));
    }

    if !extraneous.is_empty() {
        reporter.info(format!(
            "{} licenses which are not required according to dependency Cargo.toml files: {}",
            extraneous.len(),
            extraneous.join(", ")
        ));
    }

    if !unexpected.is_empty() {
        reporter.info(format!(
            "{} license files from packages that are not dependencies: {}",
            unexpected.len(),
            unexpected.join(", ")
        ));
    }

    Ok(reporter.exit_code())
}

fn missing_or_unexpected_licenses(
    dependencies: &[Package],
    licenses: &[Local],
) -> (Vec<String>, Vec<String>) {
    let expected: HashSet<_> = dependencies.iter().map(|p| p.name.clone()).collect();
    let found: HashSet<_> = licenses.iter().map(|l| l.package.clone()).collect();
    let missing: Vec<_> = sorted(expected.difference(&found).cloned().collect());
    let unexpected: Vec<_> = sorted(
        found
            .difference(&expected)
            .flat_map(|p| {
                licenses
                    .iter()
                    .filter(|l| l.package == *p)
                    .map(|l| l.file_name())
            })
            .collect(),
    );
    (missing, unexpected)
}

fn unknown_license_types(licenses: &[IdentifiedLicense]) -> Vec<String> {
    licenses
        .iter()
        .filter(|l| l.ids().next().is_none())
        .map(|l| l.license.file_name())
        .collect()
}

fn copy_left_licenses(licenses: &[IdentifiedLicense]) -> Vec<String> {
    licenses
        .iter()
        .filter(|l| l.ids().any(|l| l.is_copyleft()))
        .map(|l| l.license.file_name())
        .collect()
}

fn spdx_expressions(dependencies: &[Package]) -> anyhow::Result<Vec<Option<spdx::Expression>>> {
    dependencies
        .iter()
        .map(|package| package.project_folder.join("Cargo.toml"))
        .map(cargo_toml_spdx_expression)
        .collect()
}

fn cargo_toml_spdx_expression(path: PathBuf) -> anyhow::Result<Option<spdx::Expression>> {
    let file = std::fs::read_to_string(path)?;
    let document = file.parse::<toml_edit::Document<String>>()?;
    let text = match document
        .get("package")
        .and_then(|i| i.get("license"))
        .and_then(|i| i.as_str())
    {
        Some(text) => text,
        None => return Ok(None),
    };
    Ok(spdx::Expression::parse(text).ok())
}

fn packages_with_unmet_spdx(
    dependencies: &[Package],
    expressions: &[Option<spdx::Expression>],
    licenses: &[IdentifiedLicense],
) -> Vec<String> {
    dependencies
        .iter()
        .zip(expressions.iter())
        .filter_map(|(package, expression)| match expression {
            Some(expression) => Some((package, expression)),
            None => None,
        })
        .filter(|(package, expression)| !spdx_requirements_met(&package.name, expression, licenses))
        .map(|(package, expression)| format!("{} ({})", package.name, expression))
        .collect()
}

fn spdx_requirements_met(
    package: &str,
    expression: &spdx::Expression,
    licenses: &[IdentifiedLicense],
) -> bool {
    expression.evaluate(|requirement| match requirement.license.id() {
        Some(id) => licenses
            .iter()
            .any(|l| l.license.package == package && l.ids().any(|l| *l == id)),
        None => false,
    })
}

fn extraneous_licenses(
    dependencies: &[Package],
    expressions: &[Option<spdx::Expression>],
    licenses: &[IdentifiedLicense],
) -> Vec<String> {
    dependencies
        .iter()
        .zip(expressions.iter())
        .filter_map(|(package, expression)| match expression {
            Some(expression) => Some((package, expression)),
            None => None,
        })
        .flat_map(|(package, expression)| {
            extraneous_package_licenses(package, expression, licenses)
        })
        .collect()
}

fn extraneous_package_licenses<'a>(
    package: &Package,
    expression: &spdx::Expression,
    licenses: &[IdentifiedLicense<'a>],
) -> Vec<String> {
    let package_licenses: Vec<_> = licenses
        .iter()
        .filter(|l| l.license.package == package.name)
        .collect();
    let required = minimal_requirements(expression, &package_licenses);
    package_licenses
        .into_iter()
        .filter(|l| !required.iter().any(|r| r.name != l.license.name))
        .map(|l| format!("{} (not {})", l.license.file_name(), expression))
        .collect()
}

fn minimal_requirements<'a>(
    expression: &spdx::Expression,
    licenses: &[&IdentifiedLicense<'a>],
) -> Vec<LicenseId> {
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
    expression
        .minimized_requirements(licensee.iter())
        .into_iter()
        .flatten()
        .filter_map(|l| l.license.id())
        .collect()
}

fn sorted<T: Ord>(mut vector: Vec<T>) -> Vec<T> {
    vector.sort();
    vector
}
