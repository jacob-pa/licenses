mod arguments;
mod toml;

pub use arguments::Arguments;
pub use toml::parse_metadata_toml;

use crate::filter::Filter;
use arguments::{CheckArguments, CommonArguments, GetArguments, PruneArguments};
use clap::ValueEnum;
use std::path::PathBuf;
use toml::{CommonToml, ConfigToml};

const DEFAULT_LICENSE_DIRECTORY: &str = "./licenses";
const DEFAULT_KEYWORDS: &[&str] = &["license", "copying", "author", "copyright", "notice"];

pub fn config(toml: ConfigToml, args: Arguments) -> Config {
    match args {
        Arguments::Get(args) => Config::Get(GetConfig::new(toml, args)),
        Arguments::Check(args) => Config::Check(CheckConfig::new(toml, args)),
        Arguments::Summary(args) => Config::Summary(CommonConfig::new(toml.common, args)),
        Arguments::Prune(args) => Config::Prune(PruneConfig::new(toml, args)),
    }
}

pub enum Config {
    Get(GetConfig),
    Check(CheckConfig),
    Summary(CommonConfig),
    Prune(PruneConfig),
}

impl Config {
    pub fn common(&self) -> &CommonConfig {
        match self {
            Self::Get(config) => &config.common,
            Self::Check(config) => &config.common,
            Self::Summary(common) => common,
            Self::Prune(config) => &config.common,
        }
    }
}

pub struct GetConfig {
    pub common: CommonConfig,
    pub search_remote: SearchRemote,
    pub keywords: Vec<String>,
}

impl GetConfig {
    pub fn new(toml: ConfigToml, args: GetArguments) -> Self {
        Self {
            common: CommonConfig::new(toml.common, args.common),
            search_remote: args
                .search_remote
                .or(toml.search_remote)
                .unwrap_or_default(),
            keywords: args
                .keywords
                .or(toml.keywords)
                .unwrap_or_else(|| DEFAULT_KEYWORDS.iter().map(|s| s.to_string()).collect()),
        }
    }
}

pub struct CheckConfig {
    pub common: CommonConfig,
    pub allow: Vec<Filter>,
    pub warn: Vec<Filter>,
    pub deny: Vec<Filter>,
}

impl CheckConfig {
    pub fn new(toml: ConfigToml, args: CheckArguments) -> Self {
        Self {
            common: CommonConfig::new(toml.common, args.common),
            allow: args.allow.or(toml.allow).unwrap_or_default(),
            warn: args.warn.or(toml.warn).unwrap_or_default(),
            deny: args.deny.or(toml.deny).unwrap_or_default(),
        }
    }
}

pub struct PruneConfig {
    pub common: CommonConfig,
    pub licenses: Vec<spdx::Licensee>,
}

impl PruneConfig {
    pub fn new(toml: ConfigToml, args: PruneArguments) -> Self {
        Self {
            common: CommonConfig::new(toml.common, args.common),
            licenses: args.licenses.or(toml.licenses).unwrap_or_default(),
        }
    }
}

pub struct CommonConfig {
    pub license_directory: PathBuf,
    pub excluded: Vec<String>,
    pub build_dependencies: bool,
    pub dev_dependencies: bool,
    pub quiet: bool,
}

impl CommonConfig {
    pub fn new(toml: CommonToml, args: CommonArguments) -> Self {
        Self {
            license_directory: args
                .license_directory
                .or(toml.license_directory)
                .unwrap_or_else(|| PathBuf::from(DEFAULT_LICENSE_DIRECTORY)),
            excluded: args.excluded.or(toml.excluded).unwrap_or_default(),
            build_dependencies: args
                .build_dependencies
                .or(toml.build_dependencies)
                .unwrap_or(false),
            dev_dependencies: args
                .dev_dependencies
                .or(toml.dev_dependencies)
                .unwrap_or(false),
            quiet: args.quiet.or(toml.quiet).unwrap_or(false),
        }
    }
}

#[derive(serde::Deserialize, ValueEnum, Clone, Copy, Default)]
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
