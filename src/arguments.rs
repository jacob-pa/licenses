use clap::Parser;

use crate::config::{CommonConfig, Config, FilterConfig, KeepConfig, SearchConfig};

#[derive(Parser)]
/// A command line too for collecting and checking your dependency licenses
pub enum Command {
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
    pub fn project_directory(&self) -> &std::path::Path {
        match self {
            Self::Get(args) => &args.common.project_directory,
            Self::Check(args) => &args.common.project_directory,
            Self::Summary(args) => &args.common.project_directory,
            Self::Prune(args) => &args.common.project_directory,
        }
    }
}

#[derive(Parser)]
pub struct GetArguments {
    #[clap(flatten)]
    common: CommonConfig,

    #[clap(flatten)]
    search: SearchConfig,
}

impl GetArguments {
    pub fn overwrite(self, config: Config) -> Config {
        Config {
            common: self.common.overwrite(config.common),
            search: self.search.overwrite(config.search),
            ..config
        }
    }
}

#[derive(Parser)]
pub struct CheckArguments {
    #[clap(flatten)]
    common: CommonConfig,

    #[clap(flatten)]
    filter: FilterConfig,
}

impl CheckArguments {
    pub fn overwrite(self, config: Config) -> Config {
        Config {
            common: self.common.overwrite(config.common),
            filter: self.filter.overwrite(config.filter),
            ..config
        }
    }
}
#[derive(Parser)]
pub struct SummaryArguments {
    #[clap(flatten)]
    common: CommonConfig,
}

impl SummaryArguments {
    pub fn overwrite(self, config: Config) -> Config {
        Config {
            common: self.common.overwrite(config.common),
            ..config
        }
    }
}

#[derive(Parser)]
pub struct PruneArguments {
    #[clap(flatten)]
    common: CommonConfig,

    #[clap(flatten)]
    keep: KeepConfig,
}

impl PruneArguments {
    pub fn overwrite(self, config: Config) -> Config {
        Config {
            common: self.common.overwrite(config.common),
            keep: self.keep.overwrite(config.keep),
            ..config
        }
    }
}
