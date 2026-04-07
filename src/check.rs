use crate::Arguments;
use crate::identity::IdentifiedLicense;
use crate::local::Local;
use crate::package::Package;
use spdx::{LicenseId, LicenseItem, Licensee};
use std::collections::HashSet;
use std::process::ExitCode;

pub fn check(args: &Arguments) -> anyhow::Result<ExitCode> {
    let mut reporter = crate::report::Reporter::new(args);
    let dependencies: Vec<_> =
        crate::package::dependencies(&args.project_directory, &args.excluded)?.collect();
    let licenses = crate::local::output_folder_licenses(&args.output_directory);
    let (missing, unexpected) = missing_or_unexpected_licenses(&dependencies, &licenses);
    let licenses = crate::identity::identified_licenses(&licenses)?;
    let unknown = unknown_license_types(&licenses);
    let copy_left = copy_left_licenses(&licenses);
    let unmet_spdx = packages_with_unmet_spdx(&dependencies, &licenses);
    let extraneous = extraneous_licenses(&dependencies, &licenses);

    report_if_any(
        |m| reporter.error(m),
        &missing,
        String::to_string,
        "dependencies without any licenses",
    );

    report_if_any(
        |m| reporter.error(m),
        &unmet_spdx,
        String::to_string,
        "packages without licenses required by their Cargo.toml package.license field",
    );

    report_if_any(
        |m| reporter.error(m),
        &copy_left,
        String::to_string,
        "files with at least one copy-left license",
    );

    report_if_any(
        |m| reporter.warning(m),
        &unknown,
        String::to_string,
        "license files types with unknown types",
    );

    report_if_any(
        |m| reporter.info(m),
        &extraneous,
        String::to_string,
        "licenses which are not required according to dependency Cargo.toml files",
    );

    report_if_any(
        |m| reporter.info(m),
        &unexpected,
        String::to_string,
        "license files from packages that are not dependencies",
    );

    Ok(reporter.exit_code())
}

fn report_if_any<F, T, I>(report: F, items: &[T], item_to_string: I, message: &str)
where
    F: FnOnce(String),
    I: Fn(&T) -> String,
{
    if items.is_empty() {
        return;
    }
    let mut strings: Vec<_> = items.iter().map(item_to_string).collect();
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
