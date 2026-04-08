use crate::local::Local;
use crate::package::Package;
use crate::remote::Remote;
use crate::{Arguments, SearchRemote, local, package, remote};

pub struct Dependency {
    pub name: String,
    pub local_licenses: Vec<Local>,
    pub remote_licenses: Vec<Remote>,
}

pub fn dependencies(args: &Arguments) -> anyhow::Result<Vec<Dependency>> {
    package::dependencies(args)?
        .map(|package| package_to_dependency(package, args.search_remote))
        .collect()
}

fn package_to_dependency(
    package: Package,
    search_remote: SearchRemote,
) -> anyhow::Result<Dependency> {
    let local: Vec<_> = local::package_local_licenses(&package);
    let remote = remote_licenses(&package, &local, search_remote)?;
    Ok(Dependency {
        name: package.name,
        local_licenses: local,
        remote_licenses: remote,
    })
}

fn remote_licenses(
    package: &Package,
    local: &[Local],
    search_remote: SearchRemote,
) -> anyhow::Result<Vec<Remote>> {
    if let Some(repo_url) = &package.repository
        && should_search_remote(local, search_remote)
    {
        Ok(remote::package_remote_licenses(&package.name, repo_url)?.collect())
    } else {
        Ok(Vec::new())
    }
}

fn should_search_remote(local: &[Local], search_remote: SearchRemote) -> bool {
    matches!(
        (local.len(), search_remote),
        (0, SearchRemote::Auto) | (_, SearchRemote::Always)
    )
}
