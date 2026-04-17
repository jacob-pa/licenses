use crate::Lint;
use crate::identity::IdentifiedLicense;
use crate::report::{Level, Report};

pub fn unknown_type(licenses: &[IdentifiedLicense]) -> impl Iterator<Item = Report> {
    licenses
        .iter()
        .filter(|l| l.ids().next().is_none())
        .map(|l| l.license.file_name())
        .map(|item| Report {
            lint: Lint::UnknownType,
            level: Level::Warning,
            item,
        })
}
