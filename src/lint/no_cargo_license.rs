use crate::{
    lint::{Level, Lint, Report},
    package::Package,
};

pub fn no_cargo_license(root_package: &Package) -> Option<Report> {
    if root_package.spdx_license.is_some() {
        return None;
    }
    Some(Report {
        lint: Lint::NoCargoLicense,
        level: Level::Warning,
        item: root_package.id(),
    })
}
