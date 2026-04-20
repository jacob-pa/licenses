use crate::license;
use std::path::{Path, PathBuf};

pub struct LocalLicense(PathBuf);

impl LocalLicense {
    pub fn path(&self) -> &Path {
        &self.0
    }

    pub fn name(&self) -> String {
        self.0
            .file_name()
            .expect("invalid local license file path")
            .to_string_lossy()
            .to_string()
    }
}

pub fn package_local_licenses(keywords: &[String], project_folder: &Path) -> Vec<LocalLicense> {
    std::fs::read_dir(project_folder)
        .expect("failed to read directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| is_license(keywords, path))
        .map(LocalLicense)
        .collect()
}

#[allow(clippy::ptr_arg)]
fn is_license(keywords: &[String], path: &PathBuf) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| license::is_license(keywords, name))
        .unwrap_or(false)
}
