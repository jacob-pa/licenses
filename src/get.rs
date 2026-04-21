use crate::config::Config;
use crate::license::OutputLicense;
use crate::package::{PackageLicenses, package_licenses};
use crate::reporter::Reporter;
use cargo_metadata::Metadata;
use indicatif::ProgressIterator;
use std::path::Path;
use std::process::ExitCode;

pub fn get(metadata: Metadata, config: Config, mut reporter: Reporter) -> anyhow::Result<ExitCode> {
    let dependencies = package_licenses(&metadata, &config)?;
    let no_licenses = dependencies_with_no_licenses(&dependencies);

    reporter.info(format!(
        "{} licenses found for {} dependencies",
        total_licenses(&dependencies),
        dependencies.len()
    ));
    if !no_licenses.is_empty() {
        reporter.warning(format!(
            "{} dependencies with no licenses: {}",
            no_licenses.len(),
            no_licenses.join(", ")
        ));
    }

    std::fs::create_dir_all(&config.common.license_directory)?;
    for dependency in dependencies
        .iter()
        .progress_count(dependencies.len() as u64)
    {
        copy_local(dependency, &config.common.license_directory)?;
        copy_remote(dependency, &config.common.license_directory)?;
    }

    Ok(reporter.exit_code())
}

fn total_licenses(dependencies: &[PackageLicenses]) -> usize {
    dependencies
        .iter()
        .map(|d| d.local_licenses.len() + d.remote_licenses.len())
        .sum()
}

fn dependencies_with_no_licenses(dependencies: &[PackageLicenses]) -> Vec<String> {
    dependencies
        .iter()
        .filter(|d| d.local_licenses.is_empty() && d.remote_licenses.is_empty())
        .map(|d| d.id.to_string())
        .collect()
}

fn copy_local(dependency: &PackageLicenses, output_directory: &Path) -> anyhow::Result<()> {
    for license in &dependency.local_licenses {
        let output = OutputLicense::new(output_directory, &dependency.id, &license.name());
        std::fs::copy(license.path(), output.location)?;
    }
    Ok(())
}

fn copy_remote(dependency: &PackageLicenses, output_directory: &Path) -> anyhow::Result<()> {
    for license in &dependency.remote_licenses {
        let output = OutputLicense::new(output_directory, &dependency.id, &license.name);
        crate::license::download(license, &output.location)?;
    }
    Ok(())
}
