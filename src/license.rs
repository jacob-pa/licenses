#[derive(Debug, PartialEq)]
pub struct License<T> {
    pub package: String,
    pub name: String,
    pub location: T,
}

pub fn is_license(file_name: &str) -> bool {
    let file_name = file_name.to_lowercase();
    ["license", "copying", "authors", "copyright"]
        .into_iter()
        .any(|prefix| file_name.contains(prefix))
}
