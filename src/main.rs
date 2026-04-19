mod check;
mod config;
mod filter;
mod get;
mod identity;
mod license;
mod lint;
mod package;
mod prune;
mod report;
mod reporter;
mod summary;

use crate::filter::Filter;
use crate::lint::Lint;
use clap::{Parser, ValueEnum};
use serde::Deserialize;
use std::path::PathBuf;
use std::process::ExitCode;
use std::str::FromStr;

fn main() -> anyhow::Result<ExitCode> {
    match Command::parse() {
        Command::Get(args) => get::get(args),
        Command::Check(args) => check::check(args),
        Command::Summary(args) => summary::summary(args),
        Command::Prune(args) => prune::prune(args),
    }
}

#[derive(Parser)]
/// A command line too for collecting and checking your dependency licenses
enum Command {
    /// Collect all dependency licenses into a folder, search on disk or remotely
    Get(GetArguments),
    /// Check licenses in folder against dependencies, and report any warnings or errors
    Check(CheckArguments),
    /// Print a table to the terminal displaying a summary of dependency licenses
    Summary(CommonConfig),
    /// Prune the set of license files in the license folder to the minimum that the dependencies require
    Prune(PruneArguments),
}

#[derive(Parser)]
struct GetArguments {
    #[clap(flatten)]
    common: CommonConfig,

    #[clap(short, long, default_value = "never")]
    /// Whether to only search on disk or also remotely on github.com
    search_remote: SearchRemote,

    #[clap(
        short,
        long,
        default_values_t = ["license", "copying", "author", "copyright", "notice"].into_iter().map(|s| s.to_string()).collect::<Vec<_>>()
    )]
    /// Keywords to search for in file name to identify license files
    keywords: Vec<String>,
}

#[derive(Parser)]
struct CheckArguments {
    #[clap(flatten)]
    common: CommonConfig,

    #[clap(flatten)]
    filter: FilterConfig,
}

#[derive(Parser)]
struct PruneArguments {
    #[clap(flatten)]
    common: CommonConfig,

    /// License names in preference order to keep. Otherwise will arbitrarily prefer alphabetical (e.g. Apache-2.0 > MIT > Unlicense).
    licenses: Vec<spdx::Licensee>,
}

#[derive(Deserialize, Parser)]
struct CommonConfig {
    #[serde(default = "default_project_directory")]
    #[clap(short, long, default_value = "./")]
    /// Path to the root folder of the project to find dependencies for
    project_directory: PathBuf,
    #[serde(default = "default_output_directory")]
    #[clap(short, long, default_value = "./licenses/")]
    /// Path to the folder to store license files
    license_directory: PathBuf,
    #[serde(default)]
    #[clap(short, long)]
    /// Package names to exclude from searching for license files (and their dependencies)
    excluded: Vec<String>,
    #[serde(default)]
    #[clap(short, long, default_value_t = false)]
    /// Include dependencies only used during build
    build_dependencies: bool,
    #[serde(default)]
    #[clap(short = 'v', long, default_value_t = false)]
    /// Include dependencies only used during dev (testing)
    dev_dependencies: bool,
    #[serde(default)]
    #[clap(short, long, default_value_t = false)]
    /// Do not print any logging to stderr
    quiet: bool,
}

impl CommonConfig {
    pub fn overwrite_with(self, other: Self) -> Self {
        Self {
            project_directory: if other.project_directory != default_project_directory() {
                other.project_directory
            } else {
                self.project_directory
            },
            license_directory: if other.license_directory != default_output_directory() {
                other.license_directory
            } else {
                self.license_directory
            },
            excluded: self.excluded.into_iter().chain(other.excluded).collect(),
            build_dependencies: self.build_dependencies || other.build_dependencies,
            dev_dependencies: self.dev_dependencies || other.dev_dependencies,
            quiet: self.quiet || other.quiet,
        }
    }
}

#[derive(ValueEnum, Clone, Copy)]
enum SearchRemote {
    /// never search remotely for license files, only locally
    Never,
    /// search remotely for license files only if none are found locally
    IfNotLocal,
    /// always search remotely licenses, even if one or more found locally
    Always,
}

#[derive(Deserialize, Parser)]
pub struct FilterConfig {
    #[serde(default, deserialize_with = "filters_from_strings")]
    #[clap(short, long, value_name = "LINT_NAME[:SUB_FILTER]")]
    /// Allow violations of this specific lint, reporting as info only. Sub filters always override non-sub ones.
    pub allow: Vec<Filter>,
    #[serde(default, deserialize_with = "filters_from_strings")]
    #[clap(short, long, value_name = "LINT_NAME[:SUB_FILTER]")]
    /// Warn on violations of this specific lint. Override allow if set. Sub filters always override non-sub ones.
    pub warn: Vec<Filter>,
    #[serde(default, deserialize_with = "filters_from_strings")]
    #[clap(short, long, value_name = "LINT_NAME[:SUB_FILTER]")]
    /// Deny violations of this specific lint, reporting as an error. Overrides allow or warn if set. Sub filters always override non-sub ones.
    pub deny: Vec<Filter>,
}

impl FilterConfig {
    pub fn overwrite_with(self, other: Self) -> Self {
        Self {
            allow: self.allow.into_iter().chain(other.allow).collect(),
            warn: self.warn.into_iter().chain(other.warn).collect(),
            deny: self.deny.into_iter().chain(other.deny).collect(),
        }
    }
}

fn filters_from_strings<'de, D>(deserializer: D) -> Result<Vec<Filter>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Vec::<String>::deserialize(deserializer)?
        .into_iter()
        .map(|s| Filter::from_str(&s).map_err(serde::de::Error::custom))
        .collect()
}

fn default_project_directory() -> PathBuf {
    "./".into()
}

fn default_output_directory() -> PathBuf {
    "./licenses/".into()
}
