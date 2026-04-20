use crate::Lint;
use crate::identity::IdentifiedLicense;
use crate::report::{Level, Report};

pub fn copy_left(licenses: &[IdentifiedLicense]) -> impl Iterator<Item = Report> {
    licenses
        .iter()
        .filter(|l| l.ids().any(|l| l.is_copyleft()))
        .map(|l| Report {
            lint: Lint::CopyLeft,
            level: Level::Error,
            item: l.license.location_file_name(),
        })
}
