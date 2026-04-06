use crate::dependency::Dependency;
use crate::{Arguments, dependency, remote, warning};
use std::path::{Path, PathBuf};

pub fn get(args: &Arguments) -> anyhow::Result<()> {
    let deps =
        dependency::dependencies(&args.project_directory, &args.excluded, args.search_remote)?;
    warning::print_warnings(&deps);
    std::fs::create_dir_all(&args.output_directory)?;
    for dependency in deps {
        copy_local(args, &dependency)?;
        copy_remote(args, &dependency)?;
    }
    Ok(())
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
