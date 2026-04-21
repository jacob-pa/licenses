use crate::filter::Filter;
pub use cargo_metadata::Metadata;
use clap::{Parser, ValueEnum};
use serde::Deserialize;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct Config {
    #[serde(flatten)]
    pub common: CommonConfig,

    #[serde(flatten)]
    pub filter: FilterConfig,

    #[serde(flatten)]
    pub search: SearchConfig,

    #[serde(flatten)]
    pub keep: KeepConfig,
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
    pub fn overwrite(self, other: Self) -> Self {
        Self {
            project_directory: if self.project_directory == default_project_directory() {
                other.project_directory
            } else {
                self.project_directory
            },
            license_directory: if self.license_directory == default_output_directory() {
                other.license_directory
            } else {
                self.license_directory
            },
            excluded: other.excluded.into_iter().chain(self.excluded).collect(),
            build_dependencies: other.build_dependencies || self.build_dependencies,
            dev_dependencies: other.dev_dependencies || self.dev_dependencies,
            quiet: other.quiet || self.quiet,
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
    pub fn overwrite(self, other: Self) -> Self {
        Self {
            search_remote: self.search_remote.or(other.search_remote),
            keywords: if self.keywords.is_empty() {
                other.keywords
            } else {
                self.keywords
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
    #[serde(default, deserialize_with = "vec_from_strings")]
    #[clap(short, long, value_name = "LINT_NAME[:SUB_FILTER]")]
    /// Allow violations of this specific lint, reporting as info only. Sub filters always override non-sub ones.
    pub allow: Vec<Filter>,
    #[serde(default, deserialize_with = "vec_from_strings")]
    #[clap(short, long, value_name = "LINT_NAME[:SUB_FILTER]")]
    /// Warn on violations of this specific lint. Override allow if set. Sub filters always override non-sub ones.
    pub warn: Vec<Filter>,
    #[serde(default, deserialize_with = "vec_from_strings")]
    #[clap(short, long, value_name = "LINT_NAME[:SUB_FILTER]")]
    /// Deny violations of this specific lint, reporting as an error. Overrides allow or warn if set. Sub filters always override non-sub ones.
    pub deny: Vec<Filter>,
}

impl FilterConfig {
    pub fn overwrite(self, other: Self) -> Self {
        Self {
            allow: other.allow.into_iter().chain(self.allow).collect(),
            warn: other.warn.into_iter().chain(self.warn).collect(),
            deny: other.deny.into_iter().chain(self.deny).collect(),
        }
    }
}

fn vec_from_strings<'de, D, S>(deserializer: D) -> Result<Vec<S>, D::Error>
where
    D: serde::Deserializer<'de>,
    S: FromStr,
    S::Err: std::fmt::Display,
{
    Vec::<String>::deserialize(deserializer)?
        .into_iter()
        .map(|s| S::from_str(&s).map_err(serde::de::Error::custom))
        .collect()
}

fn default_project_directory() -> PathBuf {
    "./".into()
}

fn default_output_directory() -> PathBuf {
    "./licenses/".into()
}

#[derive(Deserialize, Parser)]
pub struct KeepConfig {
    #[serde(default, deserialize_with = "vec_from_strings")]
    /// License names in preference order to keep. Otherwise will arbitrarily prefer alphabetical (e.g. Apache-2.0 > MIT > Unlicense).
    pub(crate) licenses: Vec<spdx::Licensee>,
}

impl KeepConfig {
    pub fn overwrite(self, other: Self) -> Self {
        Self {
            licenses: if self.licenses.is_empty() {
                other.licenses
            } else {
                self.licenses
            },
        }
    }
}
