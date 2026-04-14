use crate::Lint;
use crate::identity::IdentifiedLicense;
use crate::lint::report::ReportIfAny;
use crate::lint::{Level, Report};

pub fn misnamed(licenses: &[IdentifiedLicense]) -> Option<Report> {
    licenses
        .iter()
        .filter(|l| match l.id_from_name {
            Some(id) if !l.ids_from_content.is_empty() => !l.ids_from_content.contains(&id),
            _ => false,
        })
        .map(display_misnamed)
        .report_if_any(Lint::Misnamed, Level::Warning)
}

fn display_misnamed(l: &IdentifiedLicense) -> String {
    let file_name_id = l
        .id_from_name
        .as_ref()
        .map(|i| i.base())
        .unwrap_or("<unknown>");
    let content_ids = l
        .ids_from_content
        .iter()
        .map(|i| i.base().to_string())
        .collect::<Vec<String>>()
        .join(", ");
    format!(
        "{} ({} vs {})",
        l.license.file_name(),
        file_name_id,
        content_ids
    )
}
