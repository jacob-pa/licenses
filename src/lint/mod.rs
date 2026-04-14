mod copy_left;
mod extraneous;
mod misnamed;
mod missing_or_unexpected;
mod no_licenses;
mod report;
mod unknown_type;
mod unmet_spdx;

use clap::ValueEnum;
pub use copy_left::copy_left;
use documented::DocumentedVariants;
pub use extraneous::extraneous;
pub use misnamed::misnamed;
pub use missing_or_unexpected::missing_or_unexpected;
pub use no_licenses::no_licenses;
pub use report::{Level, Report};
pub use unknown_type::unknown_type;
pub use unmet_spdx::unmet_spdx;

#[derive(Debug, Clone, Copy, ValueEnum, Hash, PartialEq, Eq, DocumentedVariants)]
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
}
