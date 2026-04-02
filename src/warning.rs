use crate::dependency::Dependency;

pub fn print_warnings(deps: &Vec<Dependency>) {
    let total_licenses = deps
        .iter()
        .map(|d| d.local_licenses.len() + d.remote_licenses.len())
        .sum::<usize>();
    let no_licenses: Vec<_> = deps
        .iter()
        .filter(|d| d.local_licenses.is_empty() && d.remote_licenses.is_empty())
        .map(|d| d.name.clone())
        .collect();
    let maybe_copyleft: Vec<_> = deps
        .iter()
        .filter(|d| only_maybe_copy_left_licenses(d))
        .map(|d| d.name.clone())
        .collect();
    println!(
        "{} licenses found for {} dependencies",
        total_licenses,
        deps.len()
    );
    if !no_licenses.is_empty() {
        println!(
            "{} dependencies with no licenses: {}",
            no_licenses.len(),
            no_licenses.join(", ")
        );
    }
    if !maybe_copyleft.is_empty() {
        println!(
            "{} dependencies with only (maybe) copyleft licenses: {}",
            maybe_copyleft.len(),
            maybe_copyleft.join(", ")
        );
    }
}

fn only_maybe_copy_left_licenses(dependency: &Dependency) -> bool {
    if dependency.local_licenses.is_empty() && dependency.remote_licenses.is_empty() {
        return false;
    }
    dependency
        .local_licenses
        .iter()
        .map(|l| &l.license_type)
        .chain(dependency.remote_licenses.iter().map(|l| &l.license_type))
        .all(|l| l.is_none())
}
