mod check;
mod config;
mod filter;
mod get;
mod identity;
mod license;
mod lint;
mod metadata;
mod package;
mod prune;
mod report;
mod reporter;
mod summary;

use crate::config::{CommonConfig, FilterConfig, KeepConfig, SearchConfig, SearchRemote};
use crate::lint::Lint;
use clap::Parser;
use std::process::ExitCode;

fn main() -> anyhow::Result<ExitCode> {
    let args = Command::parse();
    let metadata = metadata::crate_metadata(args.project_directory())?;
    let config = metadata::parse_metadata_config(&metadata)?;
    let reporter = reporter::Reporter::new(config.common.quiet);
    match args {
        Command::Get(args) => get::get(metadata, args.overwrite(config), reporter),
        Command::Check(args) => check::check(metadata, args.overwrite(config), reporter),
        Command::Summary(args) => summary::summary(args.overwrite(config)),
        Command::Prune(args) => prune::prune(metadata, args.overwrite(config), reporter),
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
    Summary(SummaryArguments),
    /// Prune the set of license files in the license folder to the minimum that the dependencies require
    Prune(PruneArguments),
}

impl Command {
    fn project_directory(&self) -> &std::path::Path {
        match self {
            Self::Get(args) => &args.common.project_directory,
            Self::Check(args) => &args.common.project_directory,
            Self::Summary(args) => &args.common.project_directory,
            Self::Prune(args) => &args.common.project_directory,
        }
    }
}

#[derive(Parser)]
struct GetArguments {
    #[clap(flatten)]
    common: CommonConfig,

    #[clap(flatten)]
    search: SearchConfig,
}

impl GetArguments {
    fn overwrite(self, config: config::Config) -> config::Config {
        config::Config {
            common: self.common.overwrite(config.common),
            search: self.search.overwrite(config.search),
            ..config
        }
    }
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
            common: self.common.overwrite(config.common),
            filter: self.filter.overwrite(config.filter),
            ..config
        }
    }
}
#[derive(Parser)]
struct SummaryArguments {
    #[clap(flatten)]
    common: CommonConfig,
}

impl SummaryArguments {
    fn overwrite(self, config: config::Config) -> config::Config {
        config::Config {
            common: self.common.overwrite(config.common),
            ..config
        }
    }
}

#[derive(Parser)]
struct PruneArguments {
    #[clap(flatten)]
    common: CommonConfig,

    #[clap(flatten)]
    keep: KeepConfig,
}

impl PruneArguments {
    fn overwrite(self, config: config::Config) -> config::Config {
        config::Config {
            common: self.common.overwrite(config.common),
            keep: self.keep.overwrite(config.keep),
            ..config
        }
    }
}
