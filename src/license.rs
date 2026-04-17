#[derive(Debug, PartialEq)]
pub struct License<T> {
    pub package: String,
    pub name: String,
    pub location: T,
}

pub fn is_license(keywords: &[String], file_name: &str) -> bool {
    let file_name = file_name.to_lowercase();
    keywords
        .iter()
        .map(|word| word.to_lowercase())
        .any(|word| file_name.contains(&word))
}
