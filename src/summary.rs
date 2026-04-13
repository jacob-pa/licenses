use crate::Arguments;
use crate::identity::IdentifiedLicense;
use itertools::Itertools;
use std::process::ExitCode;
use tabled::settings::Width;
use tabled::settings::peaker::Priority;
use tabled::{Table, Tabled, settings::Style};

pub fn summary(args: &Arguments) -> anyhow::Result<ExitCode> {
    let licenses = crate::local::output_folder_licenses(&args.output_directory);
    let licenses = crate::identity::identified_licenses(&licenses)?;
    let license_types = unique_license_types(&licenses);
    let summaries: Vec<_> = license_types
        .into_iter()
        .map(|t| license_type_summary(&t, &licenses))
        .chain(std::iter::once(unknown_license_type_summary(&licenses)))
        .collect();
    let mut table = Table::new(summaries);
    table
        .with(Style::sharp())
        .with(Width::wrap(terminal_width()).priority(Priority::max(true)));
    println!("{}", table);
    Ok(ExitCode::SUCCESS)
}

fn terminal_width() -> usize {
    terminal_size::terminal_size()
        .map(|(terminal_size::Width(w), _)| w as usize)
        .unwrap_or(200)
}

fn unique_license_types(licenses: &[IdentifiedLicense]) -> Vec<String> {
    licenses
        .iter()
        .flat_map(|l| l.ids())
        .map(|id| id.name.to_string())
        .unique()
        .sorted()
        .collect()
}

fn license_type_summary(license_type: &str, licenses: &[IdentifiedLicense]) -> LicenseSummary {
    let is_license_type =
        move |l: &&IdentifiedLicense| l.ids().map(|id| id.name).any(|name| name == license_type);
    let packages: Vec<_> = licenses
        .iter()
        .filter(is_license_type)
        .map(|l| l.license.package.clone())
        .unique()
        .sorted()
        .collect();
    let license_count = licenses.iter().filter(is_license_type).count();
    let copy_left = if spdx::license_id(license_type).unwrap().is_copyleft() {
        "copyleft"
    } else {
        "not copyleft"
    };
    LicenseSummary {
        license_type: license_type.to_string(),
        license_count,
        package_count: packages.len(),
        copyleft: copy_left.to_string(),
        packages: packages.join(", "),
    }
}

fn unknown_license_type_summary(licenses: &[IdentifiedLicense]) -> LicenseSummary {
    let is_license_type = move |l: &&IdentifiedLicense| l.ids().next().is_none();
    let no_other_licenses = |package: &str| {
        licenses
            .iter()
            .filter(|l| l.license.package == package)
            .flat_map(|l| l.ids())
            .next()
            .is_none()
    };
    let packages: Vec<_> = licenses
        .iter()
        .filter(is_license_type)
        .map(|l| l.license.package.clone())
        .unique()
        .filter(|p| no_other_licenses(p))
        .sorted()
        .collect();
    let license_count = licenses.iter().filter(is_license_type).count();
    LicenseSummary {
        license_type: "<unknown>".to_string(),
        license_count,
        package_count: packages.len(),
        copyleft: "<unknown>".to_string(),
        packages: packages.join(", "),
    }
}

#[derive(Tabled)]
struct LicenseSummary {
    #[tabled(rename = "Type")]
    license_type: String,
    #[tabled(rename = "Files")]
    license_count: usize,
    #[tabled(rename = "Packages")]
    package_count: usize,
    #[tabled(rename = "Copyleft")]
    copyleft: String,
    #[tabled(rename = "Package Names")]
    packages: String,
}
