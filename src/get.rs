use crate::package_licenses::PackageLicenses;
use crate::{GetArguments, package_licenses, remote};
use indicatif::ProgressIterator;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

pub fn get(args: &GetArguments) -> anyhow::Result<ExitCode> {
    let metadata = crate::metadata::crate_metadata(&args.common.project_directory)?;
    let mut reporter = crate::reporter::Reporter::new(args.common.quiet);
    let dependencies = package_licenses::package_licenses(args, &metadata)?;
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
    std::fs::create_dir_all(&args.common.license_directory)?;
    for dependency in dependencies
        .iter()
        .progress_count(dependencies.len() as u64)
    {
        copy_local(args, dependency)?;
        copy_remote(args, dependency)?;
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
        .map(|d| d.name.clone())
        .collect()
}

fn copy_local(args: &GetArguments, dependency: &PackageLicenses) -> anyhow::Result<()> {
    for license in &dependency.local_licenses {
        std::fs::copy(
            &license.location,
            output_file(&args.common.license_directory, dependency, &license.name),
        )?;
    }
    Ok(())
}

fn copy_remote(args: &GetArguments, dependency: &PackageLicenses) -> anyhow::Result<()> {
    for license in &dependency.remote_licenses {
        let output_path = output_file(&args.common.license_directory, dependency, &license.name);
        remote::download(&license.location, &output_path)?;
    }
    Ok(())
}

fn output_file(
    output_directory: &Path,
    dependency: &PackageLicenses,
    license_name: &str,
) -> PathBuf {
    let file_name = format!("{}-{}", dependency.name, license_name);
    output_directory.join(file_name)
}
