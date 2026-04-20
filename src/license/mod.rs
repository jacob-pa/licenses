mod local;
mod output;
mod remote;

pub use local::{LocalLicense, package_local_licenses};
pub use output::{OutputLicense, output_folder_licenses};
pub use remote::{RemoteLicense, download, package_remote_licenses};

pub fn is_license(keywords: &[String], file_name: &str) -> bool {
    let file_name = file_name.to_lowercase();
    keywords
        .iter()
        .map(|word| word.to_lowercase())
        .any(|word| file_name.contains(&word))
}
