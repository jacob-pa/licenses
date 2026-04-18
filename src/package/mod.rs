mod dependencies;
mod package_id;
mod package_licenses;

pub use dependencies::{dependencies, root_package};
pub use package_id::PackageId;
pub use package_licenses::{PackageLicenses, package_licenses};

pub use cargo_metadata::semver::Version;
pub struct Package {
    pub id: PackageId,
    pub project_folder: std::path::PathBuf,
    pub repository: Option<String>,
    pub spdx_license: Option<spdx::Expression>,
}

impl From<&cargo_metadata::Package> for Package {
    fn from(package: &cargo_metadata::Package) -> Self {
        Self {
            id: PackageId::from(package),
            repository: package.repository.clone(),
            project_folder: package
                .manifest_path
                .as_std_path()
                .parent()
                .expect("manifest not in a folder")
                .to_path_buf(),
            spdx_license: package
                .license
                .clone()
                .and_then(|l| spdx::Expression::parse(&l).ok()),
        }
    }
}
