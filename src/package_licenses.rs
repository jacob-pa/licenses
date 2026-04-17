use crate::local::Local;
use crate::metadata::Metadata;
use crate::package::Package;
use crate::remote::Remote;
use crate::{GetArguments, SearchRemote, local, package, remote};

pub struct PackageLicenses {
    pub name: String,
    pub local_licenses: Vec<Local>,
    pub remote_licenses: Vec<Remote>,
}

pub fn package_licenses(
    args: &GetArguments,
    metadata: &Metadata,
) -> anyhow::Result<Vec<PackageLicenses>> {
    package::dependencies(&args.common, metadata)
        .map(|package| package_to_dependency(args.search_remote, &args.keywords, package))
        .collect()
}

fn package_to_dependency(
    search_remote: SearchRemote,
    keywords: &[String],
    package: Package,
) -> anyhow::Result<PackageLicenses> {
    let local: Vec<_> = local::package_local_licenses(keywords, &package);
    let remote = remote_licenses(search_remote, keywords, &package, &local)?;
    Ok(PackageLicenses {
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
        (0, SearchRemote::IfNotLocal) | (_, SearchRemote::Always)
    )
}
