use strum::{AsRefStr, Display, EnumIter, IntoEnumIterator};

pub struct License<T> {
    pub name: String,
    pub location: T,
    pub license_type: Option<NonCopyLeft>,
}

pub fn is_license(file_name: &str) -> bool {
    let file_name = file_name.to_lowercase();
    ["license", "copying", "authors", "copyright"]
        .into_iter()
        .any(|prefix| file_name.contains(prefix))
}

pub fn probable_license_type(file_name: &str) -> Option<NonCopyLeft> {
    let file_name = file_name.to_lowercase();
    NonCopyLeft::iter()
        .find(|license_type| file_name.contains(&license_type.as_ref().to_lowercase()))
        .map(|license_type| Some(license_type))
        .unwrap_or(None)
}

#[derive(AsRefStr, EnumIter, Display)]
pub enum NonCopyLeft {
    Mit,
    Apache,
    Apache2,
    Unlicense,
    Zlib,
    Isc,
    Cc0,
}
