mod copy_left;
mod extraneous;
mod misnamed;
mod missing_or_unexpected;
mod no_cargo_license;
mod no_licenses;
mod unknown_type;
mod unmet_spdx;

pub use crate::report::{CombineReports, CombinedReport, Level, Report};
pub use copy_left::copy_left;
pub use extraneous::extraneous;
pub use misnamed::misnamed;
pub use missing_or_unexpected::missing_or_unexpected;
pub use no_cargo_license::no_cargo_license;
pub use no_licenses::no_licenses;
use serde::{Deserialize, Serialize};
pub use unknown_type::unknown_type;
pub use unmet_spdx::unmet_spdx;

#[derive(
    Deserialize,
    Serialize,
    Debug,
    Clone,
    Copy,
    clap::ValueEnum,
    Hash,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
    documented::DocumentedVariants,
)]
#[serde(rename_all = "kebab-case")]
pub enum Lint {
    /// Dependencies with at least one copy-left license
    CopyLeft,
    /// License files which are not required according to dependency Cargo.toml files
    Extraneous,
    /// License files with inferred types that don't match between name vs contents
    Misnamed,
    /// Dependencies without any license files
    Missing,
    /// License files from packages that are not dependencies
    Unexpected,
    /// No licenses found at all in the licenses folder for any dependency
    NoLicenses,
    /// License files types with unknown types
    UnknownType,
    /// Packages without licenses required by their Cargo.toml package.license field
    UnmetSpdx,
    /// The root package does not have the "license" field set in the Cargo.toml
    NoCargoLicense,
}
