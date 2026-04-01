mod files;
mod packages;

use clap::Parser;
use std::path::PathBuf;

fn main() {
    let args = Arguments::parse();
    let packages: Vec<_> = packages::dependencies(&args.working_directory, &args.excluded)
        .unwrap()
        .collect();
    let license_count: usize = packages
        .iter()
        .map(|p| files::license_file_paths(&p.project_folder).count())
        .sum();
    println!(
        "{license_count} licenses found in {} dependencies",
        packages.len()
    );
    for package in packages {
        if files::license_file_paths(&package.project_folder).count() == 0 {
            println!(
                "{} {} {:?}",
                package.name, package.version, package.project_folder
            );
        }
    }
}

#[derive(Parser)]
struct Arguments {
    #[clap(short, long)]
    excluded: Vec<String>,
    working_directory: PathBuf,
}
