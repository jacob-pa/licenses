use std::path::{Path, PathBuf};

pub fn license_file_paths(folder: &Path) -> impl Iterator<Item = PathBuf> {
    std::fs::read_dir(folder)
        .expect("failed to read directory")
        .filter_map(|entry| match entry {
            Ok(entry) if is_license(&entry.path()) => Some(entry.path()),
            _ => None,
        })
}

fn is_license(path: &Path) -> bool {
    let file_name = match path.file_name().and_then(|name| name.to_str()) {
        Some(file_name) => file_name,
        None => return false,
    };
    let names = [
        "license",
        "license-apache",
        "license-mit",
        "license-apache",
        "license-zlib",
        "license-cc0",
        "copying",
        "authors",
    ];
    let file_types = ["md", "txt", "apache2", "mit"];
    match file_name.split_once('.') {
        Some((prefix, suffix)) => {
            names.contains(&prefix.to_lowercase().as_str())
                && file_types.contains(&suffix.to_lowercase().as_str())
        }
        None => names.contains(&file_name.to_lowercase().as_str()),
    }
}
