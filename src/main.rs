mod check;
mod dependency;
mod get;
mod license;
mod local;
mod package;
mod remote;
mod warning;

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    match Command::parse() {
        Command::Get(args) => get::get(&args),
        Command::Check { license_directory } => check::check(&license_directory),
    }
}

#[derive(Parser)]
enum Command {
    Get(GetArguments),
    Check {
        #[clap(short, long, default_value = "./licenses/")]
        license_directory: PathBuf,
    },
}

#[derive(Parser)]
struct GetArguments {
    #[clap(short, long)]
    excluded: Vec<String>,
    #[clap(short, long, default_value = "auto")]
    search_remote: SearchRemote,
    #[clap(short, long, default_value = "./")]
    project_directory: PathBuf,
    #[clap(short, long, default_value = "./licenses/")]
    output_directory: PathBuf,
}

#[derive(ValueEnum, Clone, Copy)]
enum SearchRemote {
    Never,
    Auto,
    Always,
}
