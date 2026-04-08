use crate::dependency::Dependency;
use crate::{Arguments, dependency, remote};
use indicatif::ProgressIterator;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

pub fn get(args: &Arguments) -> anyhow::Result<ExitCode> {
    let mut reporter = crate::report::Reporter::new(args);
    let deps = dependency::dependencies(&args)?;
    let no_licenses = dependencies_with_no_licenses(&deps);
    if !no_licenses.is_empty() {
        reporter.warning(format!(
            "{} dependencies with no licenses: {}",
            no_licenses.len(),
            no_licenses.join(", ")
        ));
    }
    std::fs::create_dir_all(&args.output_directory)?;
    for dependency in deps.iter().progress_count(deps.len() as u64) {
        copy_local(args, dependency)?;
        copy_remote(args, dependency)?;
    }
    reporter.info(format!(
        "{} licenses found for {} dependencies",
        total_licenses(&deps),
        deps.len()
    ));
    Ok(reporter.exit_code())
}

fn total_licenses(dependencies: &[Dependency]) -> usize {
    dependencies
        .iter()
        .map(|d| d.local_licenses.len() + d.remote_licenses.len())
        .sum()
}

fn dependencies_with_no_licenses(dependencies: &[Dependency]) -> Vec<String> {
    dependencies
        .iter()
        .filter(|d| d.local_licenses.is_empty() && d.remote_licenses.is_empty())
        .map(|d| d.name.clone())
        .collect()
}

fn copy_local(args: &Arguments, dependency: &Dependency) -> anyhow::Result<()> {
    for license in &dependency.local_licenses {
        std::fs::copy(
            &license.location,
            output_file(&args.output_directory, dependency, &license.name),
        )?;
    }
    Ok(())
}

fn copy_remote(args: &Arguments, dependency: &Dependency) -> anyhow::Result<()> {
    for license in &dependency.remote_licenses {
        let output_path = output_file(&args.output_directory, dependency, &license.name);
        remote::download(&license.location, &output_path)?;
    }
    Ok(())
}

fn output_file(output_directory: &Path, dependency: &Dependency, license_name: &str) -> PathBuf {
    let file_name = format!("{}-{}", dependency.name, license_name);
    output_directory.join(file_name)
}
