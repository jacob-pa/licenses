use crate::Lint;
use documented::DocumentedVariants;

pub struct Report {
    pub lint: Lint,
    pub level: Level,
    pub message: String,
}
#[derive(Copy, Clone)]
pub enum Level {
    Info,
    Warning,
    Error,
}

pub trait ReportIfAny {
    fn report_if_any(self, lint: Lint, level: Level) -> Option<Report>;
}

impl<I> ReportIfAny for I
where
    I: IntoIterator<Item = String>,
{
    fn report_if_any(self, lint: Lint, level: Level) -> Option<Report> {
        let mut iterator = self.into_iter();
        let mut strings: Vec<_> = std::iter::once(iterator.next()?).chain(iterator).collect();
        strings.sort();
        let message = format!(
            "{} {}: {}",
            strings.len(),
            lint.get_variant_docs(),
            strings.join(", ")
        );
        Some(Report {
            lint,
            level,
            message,
        })
    }
}
