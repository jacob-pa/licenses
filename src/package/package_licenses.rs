use super::{Package, PackageId};
use crate::config::Metadata;
use crate::license::{LocalLicense, RemoteLicense};
use crate::{GetArguments, SearchRemote, package};

pub struct PackageLicenses {
    pub id: PackageId,
    pub local_licenses: Vec<LocalLicense>,
    pub remote_licenses: Vec<RemoteLicense>,
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
    let local = crate::license::package_local_licenses(keywords, &package.project_folder);
    let remote = remote_licenses(search_remote, keywords, &package, &local)?;
    Ok(PackageLicenses {
        id: package.id,
        local_licenses: local,
        remote_licenses: remote,
    })
}

fn remote_licenses(
    search_remote: SearchRemote,
    keywords: &[String],
    package: &Package,
    local: &[LocalLicense],
) -> anyhow::Result<Vec<RemoteLicense>> {
    if let Some(repo_url) = &package.repository
        && should_search_remote(local, search_remote)
    {
        Ok(crate::license::package_remote_licenses(keywords, repo_url)?.collect())
    } else {
        Ok(Vec::new())
    }
}

fn should_search_remote(local: &[LocalLicense], search_remote: SearchRemote) -> bool {
    matches!(
        (local.len(), search_remote),
        (0, SearchRemote::IfNotLocal) | (_, SearchRemote::Always)
    )
}
