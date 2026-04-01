mod files;
mod packages;

use clap::Parser;
use std::path::PathBuf;

fn main() {
    let args = Arguments::parse();
    for packages in packages::dependencies(&args.working_directory, &args.excluded).unwrap() {
        println!(
            "{} {} @ {:?} {:?}",
            packages.name, packages.version, packages.license, packages.license
        )
    }
}

#[derive(Parser)]
struct Arguments {
    #[clap(short, long)]
    excluded: Vec<String>,
    working_directory: PathBuf,
}
