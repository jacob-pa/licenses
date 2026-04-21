use crate::filter::Filter;
use anyhow::Context;
pub use cargo_metadata::Metadata;
use clap::{Parser, ValueEnum};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub fn crate_metadata(project_directory: &Path) -> anyhow::Result<Metadata> {
    cargo_metadata::MetadataCommand::new()
        .current_dir(project_directory)
        .exec()
        .context("failed to execute cargo metadata")
}

pub fn parse_metadata_config(metadata: &Metadata) -> anyhow::Result<Config> {
    let package = metadata
        .packages
        .iter()
        .find(|p| p.id == metadata.workspace_members[0])
        .expect("malformed metadata");
    let empty = serde_json::Value::Object(serde_json::Map::new());
    let value = package.metadata.get("licenses").unwrap_or(&empty);
    serde_json::from_value::<Config>(value.clone())
        .context("failed to parse lint rules from [package.metadata.licenses]")
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(flatten)]
    pub common: CommonConfig,

    #[serde(flatten)]
    pub filter: FilterConfig,

    #[serde(flatten)]
    pub search: SearchConfig,
}

#[derive(Deserialize, Parser)]
pub struct CommonConfig {
    #[serde(default = "default_project_directory")]
    #[clap(short, long, default_value = "./")]
    /// Path to the root folder of the project to find dependencies for
    pub project_directory: PathBuf,
    #[serde(default = "default_output_directory")]
    #[clap(short, long, default_value = "./licenses/")]
    /// Path to the folder to store license files
    pub license_directory: PathBuf,
    #[serde(default)]
    #[clap(short, long)]
    /// Package names to exclude from searching for license files (and their dependencies)
    pub excluded: Vec<String>,
    #[serde(default)]
    #[clap(short, long, default_value_t = false)]
    /// Include dependencies only used during build
    pub build_dependencies: bool,
    #[serde(default)]
    #[clap(short = 'v', long, default_value_t = false)]
    /// Include dependencies only used during dev (testing)
    pub dev_dependencies: bool,
    #[serde(default)]
    #[clap(short, long, default_value_t = false)]
    /// Do not print any logging to stderr
    pub quiet: bool,
}

impl CommonConfig {
    pub fn overwrite_with(self, other: Self) -> Self {
        Self {
            project_directory: if other.project_directory != default_project_directory() {
                other.project_directory
            } else {
                self.project_directory
            },
            license_directory: if other.license_directory != default_output_directory() {
                other.license_directory
            } else {
                self.license_directory
            },
            excluded: self.excluded.into_iter().chain(other.excluded).collect(),
            build_dependencies: self.build_dependencies || other.build_dependencies,
            dev_dependencies: self.dev_dependencies || other.dev_dependencies,
            quiet: self.quiet || other.quiet,
        }
    }
}

#[derive(Deserialize, Parser)]
pub struct SearchConfig {
    #[clap(short, long)]
    /// Whether to only search on disk or also remotely on github.com
    pub(crate) search_remote: Option<SearchRemote>,

    #[serde(default)]
    #[clap(
        short,
        long,
        default_values_t = ["license", "copying", "author", "copyright", "notice"].into_iter().map(|s| s.to_string()).collect::<Vec<_>>()
    )]
    /// Keywords to search for in file name to identify license files
    pub(crate) keywords: Vec<String>,
}

impl SearchConfig {
    pub fn overwrite_with(self, other: Self) -> Self {
        Self {
            search_remote: other.search_remote.or(self.search_remote),
            keywords: if other.keywords.is_empty() {
                self.keywords
            } else {
                other.keywords
            },
        }
    }
}

#[derive(Deserialize, ValueEnum, Clone, Copy, Default)]
#[serde(rename_all = "kebab-case")]
pub enum SearchRemote {
    #[default]
    /// never search remotely for license files, only locally
    Never,
    /// search remotely for license files only if none are found locally
    IfNotLocal,
    /// always search remotely licenses, even if one or more found locally
    Always,
}

#[derive(Deserialize, Parser)]
pub struct FilterConfig {
    #[serde(default, deserialize_with = "filters_from_strings")]
    #[clap(short, long, value_name = "LINT_NAME[:SUB_FILTER]")]
    /// Allow violations of this specific lint, reporting as info only. Sub filters always override non-sub ones.
    pub allow: Vec<Filter>,
    #[serde(default, deserialize_with = "filters_from_strings")]
    #[clap(short, long, value_name = "LINT_NAME[:SUB_FILTER]")]
    /// Warn on violations of this specific lint. Override allow if set. Sub filters always override non-sub ones.
    pub warn: Vec<Filter>,
    #[serde(default, deserialize_with = "filters_from_strings")]
    #[clap(short, long, value_name = "LINT_NAME[:SUB_FILTER]")]
    /// Deny violations of this specific lint, reporting as an error. Overrides allow or warn if set. Sub filters always override non-sub ones.
    pub deny: Vec<Filter>,
}

impl FilterConfig {
    pub fn overwrite_with(self, other: Self) -> Self {
        Self {
            allow: self.allow.into_iter().chain(other.allow).collect(),
            warn: self.warn.into_iter().chain(other.warn).collect(),
            deny: self.deny.into_iter().chain(other.deny).collect(),
        }
    }
}

fn filters_from_strings<'de, D>(deserializer: D) -> Result<Vec<Filter>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Vec::<String>::deserialize(deserializer)?
        .into_iter()
        .map(|s| Filter::from_str(&s).map_err(serde::de::Error::custom))
        .collect()
}

fn default_project_directory() -> PathBuf {
    "./".into()
}

fn default_output_directory() -> PathBuf {
    "./licenses/".into()
}
