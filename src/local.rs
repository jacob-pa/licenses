use crate::license;
use crate::license::{License, probable_license_type};
use std::path::{Path, PathBuf};

pub type Local = License<PathBuf>;

pub fn license_file_paths(folder: &Path) -> impl Iterator<Item = Local> {
    std::fs::read_dir(folder)
        .expect("failed to read directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(is_license)
        .map(|path| {
            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            Local {
                location: path,
                license_type: probable_license_type(&name),
                name,
            }
        })
}

fn is_license(path: &PathBuf) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(license::is_license)
        .unwrap_or(false)
}
