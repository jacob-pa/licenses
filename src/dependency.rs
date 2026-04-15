use crate::local::Local;
use crate::package::Package;
use crate::remote::Remote;
use crate::{GetArguments, SearchRemote, local, package, remote};

pub struct Dependency {
    pub name: String,
    pub local_licenses: Vec<Local>,
    pub remote_licenses: Vec<Remote>,
}

pub fn dependencies(args: &GetArguments) -> anyhow::Result<Vec<Dependency>> {
    package::dependencies(&args.common)?
        .map(|package| package_to_dependency(args.search_remote, &args.keywords, package))
        .collect()
}

fn package_to_dependency(
    search_remote: SearchRemote,
    keywords: &[String],
    package: Package,
) -> anyhow::Result<Dependency> {
    let local: Vec<_> = local::package_local_licenses(keywords, &package);
    let remote = remote_licenses(search_remote, keywords, &package, &local)?;
    Ok(Dependency {
        name: package.name,
        local_licenses: local,
        remote_licenses: remote,
    })
}

fn remote_licenses(
    search_remote: SearchRemote,
    keywords: &[String],
    package: &Package,
    local: &[Local],
) -> anyhow::Result<Vec<Remote>> {
    if let Some(repo_url) = &package.repository
        && should_search_remote(local, search_remote)
    {
        Ok(remote::package_remote_licenses(keywords, &package.name, repo_url)?.collect())
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
