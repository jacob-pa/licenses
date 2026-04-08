use crate::Arguments;
use crate::identity::IdentifiedLicense;
use crate::local::Local;
use crate::package::Package;
use spdx::{LicenseId, LicenseItem, Licensee};
use std::collections::HashSet;
use std::process::ExitCode;

pub fn check(args: &Arguments) -> anyhow::Result<ExitCode> {
    let mut reporter = crate::report::Reporter::new(args);
    let dependencies: Vec<_> = crate::package::dependencies(&args)?.collect();
    let licenses = crate::local::output_folder_licenses(&args.output_directory);
    let (missing, unexpected) = missing_or_unexpected_licenses(&dependencies, &licenses);
    let licenses = crate::identity::identified_licenses(&licenses)?;

    report_if_any(
        |m| reporter.error(m),
        missing,
        |l| l.to_string(),
        "dependencies without any licenses",
    );

    report_if_any(
        |m| reporter.error(m),
        packages_with_unmet_spdx(&dependencies, &licenses),
        |l| l.to_string(),
        "packages without licenses required by their Cargo.toml package.license field",
    );

    report_if_any(
        |m| reporter.error(m),
        copy_left_licenses(&licenses),
        |l| l.license.file_name(),
        "files with at least one copy-left license",
    );

    report_if_any(
        |m| reporter.warning(m),
        unknown_license_types(&licenses),
        |l| l.license.file_name(),
        "license files types with unknown types",
    );

    report_if_any(
        |m| reporter.warning(m),
        misnamed_licenses(&licenses),
        misnamed_report,
        "license files with inferred types that don't match between name vs contents",
    );

    report_if_any(
        |m| reporter.info(m),
        extraneous_licenses(&dependencies, &licenses),
        |l| l.to_string(),
        "licenses which are not required according to dependency Cargo.toml files",
    );

    report_if_any(
        |m| reporter.info(m),
        unexpected,
        |l| l.to_string(),
        "license files from packages that are not dependencies",
    );

    Ok(reporter.exit_code())
}

fn report_if_any<F, T, I>(report: F, items: Vec<T>, item_to_string: I, message: &str)
where
    F: FnOnce(String),
    I: Fn(T) -> String,
{
    if items.is_empty() {
        return;
    }
    let mut strings: Vec<_> = items.into_iter().map(item_to_string).collect();
    strings.sort();
    report(format!(
        "{} {}: {}",
        strings.len(),
        message,
        strings.join(", ")
    ));
}

fn missing_or_unexpected_licenses(
    dependencies: &[Package],
    licenses: &[Local],
) -> (Vec<String>, Vec<String>) {
    let expected: HashSet<_> = dependencies.iter().map(|p| p.name.clone()).collect();
    let found: HashSet<_> = licenses.iter().map(|l| l.package.clone()).collect();
    let missing: Vec<_> = expected.difference(&found).cloned().collect();
    let unexpected: Vec<_> = found
        .difference(&expected)
        .flat_map(|p| {
            licenses
                .iter()
                .filter(|l| l.package == *p)
                .map(|l| l.file_name())
        })
        .collect();

    (missing, unexpected)
}

fn unknown_license_types<'a>(licenses: &'a [IdentifiedLicense]) -> Vec<&'a IdentifiedLicense<'a>> {
    licenses
        .iter()
        .filter(|l| l.ids().next().is_none())
        .collect()
}

fn copy_left_licenses<'a>(licenses: &'a [IdentifiedLicense]) -> Vec<&'a IdentifiedLicense<'a>> {
    licenses
        .iter()
        .filter(|l| l.ids().any(|l| l.is_copyleft()))
        .collect()
}

fn packages_with_unmet_spdx(
    dependencies: &[Package],
    licenses: &[IdentifiedLicense],
) -> Vec<String> {
    dependencies
        .iter()
        .filter_map(|package| match &package.spdx_license {
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

fn extraneous_licenses(dependencies: &[Package], licenses: &[IdentifiedLicense]) -> Vec<String> {
    dependencies
        .iter()
        .filter_map(|package| match &package.spdx_license {
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
    match minimal_requirements(expression, &package_licenses) {
        Some(required) => package_licenses
            .into_iter()
            .filter(|l| !required.iter().any(|r| r.name != l.license.name))
            .map(|l| format!("{} (not {})", l.license.file_name(), expression))
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

fn misnamed_licenses<'a>(licenses: &'a [IdentifiedLicense<'a>]) -> Vec<&'a IdentifiedLicense<'a>> {
    licenses
        .iter()
        .filter(|l| match l.id_from_name {
            Some(id) if !l.ids_from_content.is_empty() => !l.ids_from_content.contains(&id),
            _ => false,
        })
        .collect()
}

fn misnamed_report(l: &IdentifiedLicense) -> String {
    let file_name_id = l
        .id_from_name
        .as_ref()
        .map(|i| i.base())
        .unwrap_or("<unknown>");
    let content_ids = l
        .ids_from_content
        .iter()
        .map(|i| i.base().to_string())
        .collect::<Vec<String>>()
        .join(", ");
    format!(
        "{} ({} vs {})",
        l.license.file_name(),
        file_name_id,
        content_ids
    )
}
