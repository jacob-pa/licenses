use crate::local::Local;
use crate::package::Package;
use crate::remote::Remote;
use crate::{SearchRemote, local, package, remote};
use std::path::Path;

pub struct Dependency {
    pub name: String,
    pub local_licenses: Vec<Local>,
    pub remote_licenses: Vec<Remote>,
}

pub fn dependencies(
    project_directory: &Path,
    excluded: &[String],
    search_remote: SearchRemote,
) -> anyhow::Result<Vec<Dependency>> {
    package::dependencies(project_directory, excluded)?
        .map(|package| package_to_dependency(package, search_remote))
        .collect()
}

fn package_to_dependency(
    package: Package,
    search_remote: SearchRemote,
) -> anyhow::Result<Dependency> {
    let local: Vec<_> = local::license_file_paths(&package.project_folder);
    let remote = remote_licenses(&package.repository, &local, search_remote)?;
    Ok(Dependency {
        name: package.name,
        local_licenses: local,
        remote_licenses: remote,
    })
}

fn remote_licenses(
    repo_url: &Option<String>,
    local: &Vec<Local>,
    search_remote: SearchRemote,
) -> anyhow::Result<Vec<Remote>> {
    if let Some(repo_url) = repo_url
        && should_search_remote(local, search_remote)
    {
        Ok(remote::license_file_urls(repo_url)?.collect())
    } else {
        Ok(Vec::new())
    }
}

fn should_search_remote(local: &Vec<Local>, search_remote: SearchRemote) -> bool {
    match (local.len(), search_remote) {
        (0, SearchRemote::Auto) | (_, SearchRemote::Always) => true,
        _ => false,
    }
}
