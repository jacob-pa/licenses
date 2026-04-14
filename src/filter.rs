use crate::lint::{Level, Report};
use crate::{Arguments, Lint};
use std::collections::HashMap;

pub struct FilterRules {
    lint_level: HashMap<Lint, Level>,
}

impl FilterRules {
    pub fn new(args: &Arguments) -> Self {
        Self {
            lint_level: args
                .allow
                .iter()
                .map(|lint| (*lint, Level::Info))
                .chain(args.warn.iter().map(|lint| (*lint, Level::Warning)))
                .chain(args.deny.iter().map(|lint| (*lint, Level::Error)))
                .collect(),
        }
    }

    pub fn filter(&self, mut report: Report) -> Option<Report> {
        if let Some(level) = self.lint_level.get(&report.lint) {
            report.level = *level;
        }
        Some(report)
    }
}
