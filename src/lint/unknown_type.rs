use super::report::{Level, Report, ReportIfAny};
use crate::Lint;
use crate::identity::IdentifiedLicense;

pub fn unknown_type(licenses: &[IdentifiedLicense]) -> Option<Report> {
    licenses
        .iter()
        .filter(|l| l.ids().next().is_none())
        .map(|l| l.license.file_name())
        .report_if_any(Lint::UnknownType, Level::Warning)
}
