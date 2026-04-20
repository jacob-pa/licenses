use crate::package::{PackageId, Version};
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq)]
pub struct OutputLicense {
    pub package_id: PackageId,
    pub name: String,
    pub location: PathBuf,
}

impl OutputLicense {
    pub fn new(output_directory: &Path, package_id: &PackageId, name: &str) -> Self {
        Self {
            package_id: package_id.clone(),
            name: name.to_string(),
            location: output_directory.join(format!(
                "{}_{}_{}",
                package_id.name, package_id.version, name
            )),
        }
    }

    pub fn location_file_name(&self) -> String {
        self.location
            .file_name()
            .expect("invalid local license file path")
            .to_string_lossy()
            .to_string()
    }
}

pub fn output_folder_licenses(project_folder: &Path) -> Vec<OutputLicense> {
    let entries = match std::fs::read_dir(project_folder) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Vec::new(),
        Err(error) => panic!("failed to read directory: {}", error),
    };
    entries
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter_map(from_output_folder)
        .collect()
}

fn from_output_folder(location: PathBuf) -> Option<OutputLicense> {
    let (package, suffix) = location.file_name()?.to_str()?.split_once('_')?;
    let (version, name) = suffix.split_once('_')?;

    Some(OutputLicense {
        package_id: PackageId::new(package, Version::parse(version).ok()?),
        name: name.to_string(),
        location,
    })
}
