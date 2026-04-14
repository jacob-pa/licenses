pub struct Report {
    pub level: Level,
    pub message: String,
}

pub enum Level {
    Info,
    Warning,
    Error,
}

pub trait ReportIfAny<T> {
    fn report_if_any(
        self,
        level: Level,
        message: &str,
        item_to_string: impl Fn(T) -> String,
    ) -> Option<Report>;
}

impl<T, I> ReportIfAny<T> for I
where
    I: IntoIterator<Item = T>,
{
    fn report_if_any(
        self,
        level: Level,
        message: &str,
        item_to_string: impl Fn(T) -> String,
    ) -> Option<Report> {
        let mut iterator = self.into_iter();
        let items = std::iter::once(iterator.next()?).chain(iterator);
        let mut strings: Vec<_> = items.map(item_to_string).collect();
        strings.sort();
        let message = format!("{} {}: {}", strings.len(), message, strings.join(", "));
        Some(Report { level, message })
    }
}
