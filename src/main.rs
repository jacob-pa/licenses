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

use crate::config::{CommonConfig, FilterConfig, SearchRemote};
use crate::lint::Lint;
use clap::Parser;
use std::process::ExitCode;

fn main() -> anyhow::Result<ExitCode> {
    let args = Command::parse();
    let metadata = config::crate_metadata(args.project_directory())?;
    let config = config::parse_metadata_config(&metadata)?;
    let reporter = reporter::Reporter::new(config.common.quiet);
    match args {
        Command::Get(args) => get::get(args),
        Command::Check(args) => check::check(metadata, args.overwrite(config), reporter),
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

impl Command {
    fn project_directory(&self) -> &std::path::Path {
        match self {
            Self::Get(args) => &args.common.project_directory,
            Self::Check(args) => &args.common.project_directory,
            Self::Summary(args) => &args.project_directory,
            Self::Prune(args) => &args.common.project_directory,
        }
    }
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

impl CheckArguments {
    fn overwrite(self, config: config::Config) -> config::Config {
        config::Config {
            common: config.common.overwrite_with(self.common),
            filter: config.filter.overwrite_with(self.filter),
        }
    }
}

#[derive(Parser)]
struct PruneArguments {
    #[clap(flatten)]
    common: CommonConfig,

    /// License names in preference order to keep. Otherwise will arbitrarily prefer alphabetical (e.g. Apache-2.0 > MIT > Unlicense).
    licenses: Vec<spdx::Licensee>,
}
