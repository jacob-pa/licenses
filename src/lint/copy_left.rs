use super::report::{Level, Report, ReportIfAny};
use crate::Lint;
use crate::identity::IdentifiedLicense;

pub fn copy_left(licenses: &[IdentifiedLicense]) -> Option<Report> {
    licenses
        .iter()
        .filter(|l| l.ids().any(|l| l.is_copyleft()))
        .map(|l| l.license.file_name())
        .report_if_any(Lint::CopyLeft, Level::Error)
}
