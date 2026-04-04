use spdx::LicenseId;
use std::path::{Path, PathBuf};

pub fn check(license_directory: &Path) -> anyhow::Result<()> {
    let licenses: Vec<_> = files(license_directory).map(path_to_license).collect();
    let unknown: Vec<_> = licenses
        .iter()
        .filter(|l| l.id_from_name.is_none())
        .map(|l| l.name.clone())
        .collect();
    println!(
        "{} unknown license types out of {}: {}",
        unknown.len(),
        licenses.len(),
        unknown.join(", ")
    );
    Ok(())
}

struct License {
    name: String,
    id_from_name: Option<LicenseId>,
    path: PathBuf,
}

fn path_to_license(path: PathBuf) -> License {
    License {
        name: path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split_once('-')
            .unwrap()
            .1
            .to_string(),
        id_from_name: id_from_name(&path),
        path,
    }
}

fn id_from_name(path: &Path) -> Option<LicenseId> {
    path.file_name()?
        .to_str()?
        .split('-')
        .filter_map(|word| spdx::imprecise_license_id(word).map(|(id, _)| id))
        .next()
}

fn files(license_directory: &Path) -> impl Iterator<Item = PathBuf> {
    std::fs::read_dir(license_directory)
        .expect("failed to read directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
}
