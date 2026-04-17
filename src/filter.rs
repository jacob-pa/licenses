use crate::lint::{Level, Report};
use crate::metadata::Config;
use crate::{CheckArguments, Filter, Lint};
use std::collections::HashMap;

pub struct FilterRules {
    rules: HashMap<Lint, Level>,
    sub_rules: HashMap<(Lint, String), Level>,
}

impl FilterRules {
    pub fn new(config: &Config, args: &CheckArguments) -> Self {
        Self {
            rules: rules(config, args),
            sub_rules: sub_rules(config, args),
        }
    }

    pub fn filter(&self, mut report: Report) -> Option<Report> {
        if let Some(level) = self.sub_rules.get(&(report.lint, report.item.to_string())) {
            report.level = *level;
        } else if let Some(level) = self.rules.get(&report.lint) {
            report.level = *level;
        }
        Some(report)
    }
}

fn rules(config: &Config, args: &CheckArguments) -> HashMap<Lint, Level> {
    filter_levels(config, args)
        .filter(|(filter, _)| filter.sub_filter.is_none())
        .map(|(filter, level)| (filter.lint, level))
        .collect()
}

fn sub_rules(config: &Config, args: &CheckArguments) -> HashMap<(Lint, String), Level> {
    filter_levels(config, args)
        .filter_map(|(filter, level)| {
            filter
                .sub_filter
                .as_ref()
                .map(|sub_filter| ((filter.lint, sub_filter.clone()), level))
        })
        .collect()
}

fn filter_levels<'a>(
    config: &'a Config,
    args: &'a CheckArguments,
) -> impl Iterator<Item = (Filter, Level)> {
    config
        .allow
        .iter()
        .map(|lint| {
            (
                Filter {
                    lint: lint.clone(),
                    sub_filter: None,
                },
                Level::Info,
            )
        })
        .chain(
            args.allow
                .iter()
                .map(|filter| (filter.clone(), Level::Info)),
        )
        .chain(
            args.warn
                .iter()
                .map(|filter| (filter.clone(), Level::Warning)),
        )
        .chain(
            args.deny
                .iter()
                .map(|filter| (filter.clone(), Level::Error)),
        )
}
