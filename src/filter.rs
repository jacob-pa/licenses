use crate::lint::{Level, Report};
use crate::{CheckArguments, FilterConfig, Lint};
use clap::ValueEnum;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone)]
pub struct Filter {
    pub lint: Lint,
    pub sub_filter: Option<String>,
}

impl FromStr for Filter {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let (lint, sub_filter) = match string.split_once(":") {
            Some((prefix, suffix)) => (prefix, Some(suffix.to_string())),
            None => (string, None),
        };
        Ok(Self {
            lint: Lint::from_str(lint, true)?,
            sub_filter,
        })
    }
}

pub struct FilterRules {
    rules: HashMap<Lint, Level>,
    sub_rules: HashMap<(Lint, String), Level>,
}

impl FilterRules {
    pub fn new(config: &FilterConfig, args: &CheckArguments) -> Self {
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

fn rules(config: &FilterConfig, args: &CheckArguments) -> HashMap<Lint, Level> {
    filter_levels(config, args)
        .filter(|(filter, _)| filter.sub_filter.is_none())
        .map(|(filter, level)| (filter.lint, level))
        .collect()
}

fn sub_rules(config: &FilterConfig, args: &CheckArguments) -> HashMap<(Lint, String), Level> {
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
    config: &'a FilterConfig,
    args: &'a CheckArguments,
) -> impl Iterator<Item = (Filter, Level)> {
    filter_level(&config.allow, Level::Info)
        .chain(filter_level(&args.filters.allow, Level::Info))
        .chain(filter_level(&config.warn, Level::Warning))
        .chain(filter_level(&args.filters.warn, Level::Warning))
        .chain(filter_level(&config.deny, Level::Error))
        .chain(filter_level(&args.filters.deny, Level::Error))
}

fn filter_level(filters: &[Filter], level: Level) -> impl Iterator<Item = (Filter, Level)> {
    filters.iter().map(move |filter| (filter.clone(), level))
}
