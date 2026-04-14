use crate::Arguments;
use crate::interrupt::Interrupt;
use crate::package::Package;
use crates_io_api::SyncClient;
use indicatif::{ProgressBar, ProgressIterator};
use jiff::Timestamp;
use std::collections::HashMap;
use std::path::Path;
use std::process::ExitCode;
use url::Url;

pub fn review(args: &Arguments) -> anyhow::Result<ExitCode> {
    let interrupt = Interrupt::setup()?;
    let metadata_file = args.license_directory.join(".metadata.json");
    let metadata = load_from_file(&metadata_file)?;
    let dependencies: Vec<_> = crate::package::dependencies(args)?.collect();
    let missing_deps = missing_dependencies(&metadata, dependencies);
    let new_metadata = fetch_missing_metadata(interrupt, metadata.len(), missing_deps)?;
    write_to_file(&combine_metadata(metadata, new_metadata), &metadata_file)?;
    Ok(ExitCode::SUCCESS)
}

type Metadata = HashMap<String, CrateMetadata>;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CrateMetadata {
    home: Option<Url>,
    docs: Option<Url>,
    repo: Option<Url>,
    versions: u64,
    downloads: u64,
    dependents: u64,
    created: Timestamp,
}

fn load_from_file(path: &Path) -> anyhow::Result<Metadata> {
    if !path.exists() {
        return Ok(Metadata::new());
    }
    Ok(serde_json::from_str(&std::fs::read_to_string(path)?)?)
}

fn missing_dependencies(metadata: &Metadata, dependencies: Vec<Package>) -> Vec<Package> {
    dependencies
        .into_iter()
        .filter(|p| !metadata.contains_key(&p.name))
        .collect()
}

fn fetch_missing_metadata(
    interrupt: Interrupt,
    completed: usize,
    dependencies: Vec<Package>,
) -> anyhow::Result<Metadata> {
    let user_agent = "trusted-deps-metadata (jacob.araiza@paconsulting.com)";
    let delay = std::time::Duration::from_secs(1);
    let client = SyncClient::new(user_agent, delay)?;
    let total = completed + dependencies.len();
    let progress_bar = ProgressBar::new(total as u64).with_position(completed as u64);
    dependencies
        .into_iter()
        .take_while(|_| interrupt.should_continue())
        .map(|p| Ok((p.name.clone(), crate_metadata(&client, &p.name)?)))
        .progress_with(progress_bar)
        .collect()
}

fn crate_metadata(client: &SyncClient, name: &str) -> anyhow::Result<CrateMetadata> {
    let data = client.get_crate(name)?.crate_data;
    Ok(CrateMetadata {
        home: data.homepage.map(|s| Url::parse(&s)).transpose()?,
        docs: data.documentation.map(|s| Url::parse(&s)).transpose()?,
        repo: data.repository.map(|s| Url::parse(&s)).transpose()?,
        versions: data.versions.map(|v| v.len() as u64).unwrap_or(0),
        downloads: data.downloads,
        dependents: client.crate_reverse_dependency_count(name)?,
        created: Timestamp::from_second(data.created_at.timestamp())?,
    })
}

fn combine_metadata(old: Metadata, new: Metadata) -> Metadata {
    old.into_iter().chain(new).collect()
}

fn write_to_file(metadata: &Metadata, path: &Path) -> anyhow::Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path.parent().unwrap())?;
    }
    std::fs::write(path, serde_json::to_string(metadata)?)?;
    Ok(())
}
