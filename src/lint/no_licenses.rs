use crate::identity::IdentifiedLicense;
use crate::lint::{Level, Report};
use crate::package::Package;
use std::path::Path;

pub fn no_licenses(
    license_directory: &Path,
    dependencies: &[Package],
    licenses: &[IdentifiedLicense],
) -> Option<Report> {
    if dependencies.is_empty() || !licenses.is_empty() {
        return None;
    }

    Some(Report {
        level: Level::Warning,
        message: format!(
            "no licenses found at all in '{}'",
            license_directory.display()
        ),
    })
}
