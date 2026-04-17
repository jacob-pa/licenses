mod check;
mod filter;
mod get;
mod identity;
mod license;
mod lint;
mod local;
mod metadata;
mod package;
mod package_licenses;
mod remote;
mod reporter;
mod summary;

use crate::lint::Lint;
use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use std::process::ExitCode;
use std::str::FromStr;

fn main() -> anyhow::Result<ExitCode> {
    match Command::parse() {
        Command::Get(args) => get::get(&args),
        Command::Check(args) => check::check(&args),
        Command::Summary(args) => summary::summary(&args),
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
    Summary(Arguments),
}

#[derive(Parser)]
struct GetArguments {
    #[clap(flatten)]
    common: Arguments,

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
    common: Arguments,

    #[clap(short, long, value_name = "LINT_NAME[:SUB_FILTER]")]
    /// Allow violations of this specific lint, reporting as info only. Sub filters always override non-sub ones.
    allow: Vec<Filter>,
    #[clap(short, long, value_name = "LINT_NAME[:SUB_FILTER]")]
    /// Warn on violations of this specific lint. Override allow if set. Sub filters always override non-sub ones.
    warn: Vec<Filter>,
    #[clap(short, long, value_name = "LINT_NAME[:SUB_FILTER]")]
    /// Deny violations of this specific lint, reporting as an error. Overrides allow or warn if set. Sub filters always override non-sub ones.
    deny: Vec<Filter>,
}

#[derive(Parser)]
struct Arguments {
    #[clap(short, long, default_value = "./")]
    /// Path to the root folder of the project to find dependencies for
    project_directory: PathBuf,
    #[clap(short, long, default_value = "./licenses/")]
    /// Path to the folder to store license files
    license_directory: PathBuf,
    #[clap(short, long)]
    /// Package names to exclude from searching for license files (and their dependencies)
    excluded: Vec<String>,
    #[clap(short, long, default_value_t = false)]
    /// Include dependencies only used during build
    build_dependencies: bool,
    #[clap(short = 'v', long, default_value_t = false)]
    /// Include dependencies only used during dev (testing)
    dev_dependencies: bool,
    #[clap(short, long, default_value_t = false)]
    /// Do not print any logging to stderr
    quiet: bool,
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

#[derive(Clone)]
struct Filter {
    lint: Lint,
    sub_filter: Option<String>,
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
