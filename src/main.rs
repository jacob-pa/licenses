mod check;
mod dependency;
mod filter;
mod get;
mod identity;
mod interrupt;
mod license;
mod lint;
mod local;
mod package;
mod remote;
mod reporter;
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
/// A command line too for collecting and checking your dependency licenses
enum Command {
    /// Collect all dependency licenses into a folder, search on disk or remotely
    Get(Arguments),
    /// Check licenses in folder against dependencies, and report any warnings or errors
    Check(Arguments),
    /// Print a table to the terminal displaying a summary of dependency licenses
    Summary(Arguments),
    Review(Arguments),
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
    #[clap(short, long, default_value = "auto")]
    /// Whether to only search on disk or also remotely on github.com
    search_remote: SearchRemote,
    #[clap(short, long, default_value = "false")]
    /// Include dependencies only used during build
    build_dependencies: bool,
    #[clap(short = 'v', long, default_value = "false")]
    /// Include dependencies only used during dev (testing)
    dev_dependencies: bool,
    #[clap(short, long, default_value = "false")]
    /// Do not print any logging to stderr
    quiet: bool,
    #[clap(short, long)]
    /// Allow violations of this specific lint, reporting as info only.
    allow: Vec<Lint>,
    #[clap(short, long)]
    /// Warn on violations of this specific lint. Override allow if set.
    warn: Vec<Lint>,
    #[clap(short, long)]
    /// Deny violations of this specific lint, reporting as an error. Overrides allow or warn if set.
    deny: Vec<Lint>,
}

#[derive(ValueEnum, Clone, Copy)]
enum SearchRemote {
    /// never search remotely for license files, only locally
    Never,
    /// search remotely for license files only if none are found locally
    Auto,
    /// always search remotely licenses, even if one or more found locally
    Always,
}

#[derive(Debug, Clone, Copy, ValueEnum, Hash, PartialEq, Eq)]
enum Lint {
    CopyLeft,
    Extraneous,
    Misnamed,
    Missing,
    Unexpected,
    NoLicenses,
    UnknownType,
    UnmetSpdx,
}
