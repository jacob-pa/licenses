use crate::Lint;
use documented::DocumentedVariants;
use itertools::Itertools;
use std::fmt::{Display, Formatter};

pub struct Report {
    pub lint: Lint,
    pub level: Level,
    pub item: String,
}

pub struct CombinedReport {
    pub lint: Lint,
    pub level: Level,
    pub items: Vec<String>,
}

impl Display for CombinedReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {} {}: {}",
            serde_json::to_value(self.lint).unwrap().as_str().unwrap(),
            self.items.len(),
            self.lint.get_variant_docs(),
            self.items.join(", ")
        )
    }
}

// NOTE variant order is important for PartialOrd
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub enum Level {
    Info,
    Warning,
    Error,
}

pub trait CombineReports {
    fn combine_reports(self) -> impl Iterator<Item = CombinedReport>;
}

impl<I> CombineReports for I
where
    I: IntoIterator<Item = Report>,
{
    fn combine_reports(self) -> impl Iterator<Item = CombinedReport> {
        self.into_iter()
            .into_group_map_by(|r| (r.lint, r.level))
            .into_iter()
            .map(|(key, group)| (key, group.into_iter().map(|r| r.item).sorted().collect()))
            .map(|((lint, level), items)| CombinedReport { lint, level, items })
    }
}
