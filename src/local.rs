use crate::license;
use crate::license::License;
use crate::package::Package;
use std::path::{Path, PathBuf};

pub type Local = License<PathBuf>;

pub fn package_local_licenses(package: &Package) -> Vec<Local> {
    std::fs::read_dir(&package.project_folder)
        .expect("failed to read directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(is_license)
        .map(|path| {
            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            Local {
                package: package.name.clone(),
                location: path,
                name,
            }
        })
        .collect()
}

pub fn output_folder_licenses(project_folder: &Path) -> Vec<Local> {
    std::fs::read_dir(project_folder)
        .expect("failed to read directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter_map(|path| {
            let (package, name) = path.file_name()?.to_str()?.split_once('-')?;
            Some(Local {
                package: package.to_string(),
                location: path.to_path_buf(),
                name: name.to_string(),
            })
        })
        .collect()
}

fn is_license(path: &PathBuf) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(license::is_license)
        .unwrap_or(false)
}
