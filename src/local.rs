use crate::license;
use crate::license::License;
use crate::package::Package;
use std::path::{Path, PathBuf};

pub type Local = License<PathBuf>;

impl License<PathBuf> {
    pub fn file_name(&self) -> String {
        self.location
            .file_name()
            .expect("invalid local license file path")
            .to_string_lossy()
            .to_string()
    }
}

pub fn package_local_licenses(keywords: &[String], package: &Package) -> Vec<Local> {
    std::fs::read_dir(&package.project_folder)
        .expect("failed to read directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| is_license(keywords, path))
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
    let entries = match std::fs::read_dir(project_folder) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Vec::new(),
        Err(error) => panic!("failed to read directory: {}", error),
    };
    entries
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

#[allow(clippy::ptr_arg)]
fn is_license(keywords: &[String], path: &PathBuf) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| license::is_license(keywords, name))
        .unwrap_or(false)
}
