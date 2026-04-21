use crate::{config::SearchRemote, filter::Filter};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
/// A command line too for collecting and checking your dependency licenses
pub enum Arguments {
    /// Collect all dependency licenses into a folder, search on disk or remotely
    Get(GetArguments),
    /// Check licenses in folder against dependencies, and report any warnings or errors
    Check(CheckArguments),
    /// Print a table to the terminal displaying a summary of dependency licenses
    Summary(CommonArguments),
    /// Prune the set of license files in the license folder to the minimum that the dependencies require
    Prune(PruneArguments),
}

impl Arguments {
    pub fn common(&self) -> &CommonArguments {
        match self {
            Self::Get(args) => &args.common,
            Self::Check(args) => &args.common,
            Self::Summary(common) => common,
            Self::Prune(args) => &args.common,
        }
    }
}

#[derive(Parser)]
pub struct GetArguments {
    #[clap(flatten)]
    pub common: CommonArguments,

    #[clap(short, long, default_value = "if-not-local")]
    /// Whether to only search on disk or also remotely on github.com
    pub search_remote: Option<SearchRemote>,

    #[clap(short, long)]
    /// Keywords to search for in file name to identify license files
    pub keywords: Option<Vec<String>>,
}

#[derive(Parser)]
pub struct CheckArguments {
    #[clap(flatten)]
    pub common: CommonArguments,

    #[clap(short, long, value_name = "LINT_NAME[:SUB_FILTER]")]
    /// Allow violations of this specific lint, reporting as info only. Sub filters always override non-sub ones.
    pub allow: Option<Vec<Filter>>,

    #[clap(short, long, value_name = "LINT_NAME[:SUB_FILTER]")]
    /// Warn on violations of this specific lint. Override allow if set. Sub filters always override non-sub ones.
    pub warn: Option<Vec<Filter>>,

    #[clap(short, long, value_name = "LINT_NAME[:SUB_FILTER]")]
    /// Deny violations of this specific lint, reporting as an error. Overrides allow or warn if set. Sub filters always override non-sub ones.
    pub deny: Option<Vec<Filter>>,
}

#[derive(Parser)]
pub struct PruneArguments {
    #[clap(flatten)]
    pub common: CommonArguments,

    /// License names in preference order to keep. Otherwise will arbitrarily prefer alphabetical (e.g. Apache-2.0 > MIT > Unlicense).
    pub licenses: Option<Vec<spdx::Licensee>>,
}

#[derive(Parser)]
pub struct CommonArguments {
    #[clap(short, long, default_value = "./")]
    /// Path to the root folder of the project to find dependencies for
    pub project_directory: PathBuf,

    #[clap(short, long)]
    /// Path to the folder to store license files
    pub license_directory: Option<PathBuf>,

    #[clap(short, long)]
    /// Package names to exclude from searching for license files (and their dependencies)
    pub excluded: Option<Vec<String>>,

    #[clap(short, long)]
    /// Include dependencies only used during build
    pub build_dependencies: Option<bool>,

    #[clap(short = 'v', long)]
    /// Include dependencies only used during dev (testing)
    pub dev_dependencies: Option<bool>,

    #[clap(short, long)]
    /// Do not print any logging to stderr
    pub quiet: Option<bool>,
}
