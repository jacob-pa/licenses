use crate::package::Version;

#[derive(Debug, PartialEq)]
pub struct License<T> {
    pub package: String,
    pub version: Version,
    pub name: String,
    pub location: T,
}

impl<T> License<T> {
    pub fn package_id(&self) -> String {
        format!("{}_{}", self.package, self.version)
    }
}

pub fn is_license(keywords: &[String], file_name: &str) -> bool {
    let file_name = file_name.to_lowercase();
    keywords
        .iter()
        .map(|word| word.to_lowercase())
        .any(|word| file_name.contains(&word))
}
