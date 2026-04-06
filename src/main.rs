mod check;
mod dependency;
mod get;
mod identity;
mod license;
mod local;
mod package;
mod remote;
mod report;

use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> anyhow::Result<ExitCode> {
    match Command::parse() {
        Command::Get(args) => get::get(&args),
        Command::Check(args) => check::check(&args),
    }
}

#[derive(Parser)]
enum Command {
    Get(Arguments),
    Check(Arguments),
}

#[derive(Parser)]
struct Arguments {
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
