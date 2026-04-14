use super::report::{Level, Report, ReportIfAny};
use crate::identity::IdentifiedLicense;

pub fn unknown_type(licenses: &[IdentifiedLicense]) -> Option<Report> {
    licenses
        .iter()
        .filter(|l| l.ids().next().is_none())
        .report_if_any(
            Level::Warning,
            "license files types with unknown types",
            |l| l.license.file_name(),
        )
}
