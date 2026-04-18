pub use crate::package::Version;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct PackageId {
    pub name: String,
    pub version: Version,
}

impl PackageId {
    pub fn new(name: &str, version: Version) -> Self {
        Self {
            name: name.replace('_', "-"), // underscores used to seperate version and license file name
            version,
        }
    }
}

impl From<&cargo_metadata::Package> for PackageId {
    fn from(package: &cargo_metadata::Package) -> Self {
        Self::new(&package.name, package.version.clone())
    }
}

impl std::fmt::Display for PackageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.name, self.version)
    }
}
