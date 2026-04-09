mod check;
mod dependency;
mod get;
mod identity;
mod interrupt;
mod license;
mod local;
mod package;
mod remote;
mod report;
mod review;
mod summary;

use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> anyhow::Result<ExitCode> {
    match Command::parse() {
        Command::Get(args) => get::get(&args),
        Command::Check(args) => check::check(&args),
        Command::Summary(args) => summary::summary(&args),
        Command::Review(args) => review::review(&args),
    }
}

#[derive(Parser)]
enum Command {
    Get(Arguments),
    Check(Arguments),
    Summary(Arguments),
    Review(Arguments),
}

#[derive(Parser)]
struct Arguments {
    #[clap(short, long, default_value = "./")]
    project_directory: PathBuf,
    #[clap(short, long, default_value = "./licenses/")]
    output_directory: PathBuf,
    #[clap(short, long)]
    excluded: Vec<String>,
    #[clap(short, long, default_value = "auto")]
    search_remote: SearchRemote,
    #[clap(short, long, default_value = "false")]
    build_dependencies: bool,
    #[clap(short, long, default_value = "false")]
    dev_dependencies: bool,
    #[clap(short, long, default_value = "false")]
    quiet: bool,
    #[clap(short = 'w', long, default_value = "false")]
    error_on_warning: bool,
}

#[derive(ValueEnum, Clone, Copy)]
enum SearchRemote {
    Never,
    Auto,
    Always,
}
