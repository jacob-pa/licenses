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

use crate::arguments::Command;
use crate::config::{CommonConfig, FilterConfig, SearchRemote};
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
