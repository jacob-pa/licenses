mod arguments;
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

use crate::arguments::Arguments;
use crate::config::{CommonConfig, Config, SearchRemote};
use crate::lint::Lint;
use clap::Parser;
use std::process::ExitCode;

fn main() -> anyhow::Result<ExitCode> {
    let args = Arguments::parse();
    let metadata = metadata::crate_metadata(&args.common().project_directory)?;
    let toml_config = metadata::parse_metadata_config(&metadata)?;
    let config = config::config(toml_config, args);
    let reporter = reporter::Reporter::new(config.common().quiet);
    match config {
        Config::Get(config) => get::get(metadata, config, reporter),
        Config::Check(config) => check::check(metadata, config, reporter),
        Config::Summary(config) => summary::summary(config),
        Config::Prune(config) => prune::prune(metadata, config, reporter),
    }
}
