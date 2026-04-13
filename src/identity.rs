use crate::local::Local;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use spdx::LicenseId;
use spdx::detection::LicenseType;
use spdx::detection::scan::Scanner;

#[derive(Debug, PartialEq)]
pub struct IdentifiedLicense<'a> {
    pub license: &'a Local,
    pub id_from_name: Option<LicenseId>,
    pub ids_from_content: Vec<LicenseId>,
}

impl IdentifiedLicense<'_> {
    pub fn ids(&self) -> impl Iterator<Item = &LicenseId> {
        self.ids_from_content.iter().chain(&self.id_from_name)
    }
}

pub fn identified_licenses(licenses: &'_ [Local]) -> anyhow::Result<Vec<IdentifiedLicense<'_>>> {
    let mut store = spdx::detection::Store::load_inline()?;
    store.add_variant(
        "Apache-2.0",
        LicenseType::Alternate,
        include_str!("../tests/anyhow-LICENSE-APACHE").into(),
    )?;
    let scanner = Scanner::new(&store).optimize(true /* look for multi-license files */);
    licenses
        .par_iter()
        .progress_count(licenses.len() as u64)
        .map(|license| identify_license(&scanner, license))
        .collect()
}

fn identify_license<'a>(
    scanner: &Scanner,
    license: &'a Local,
) -> anyhow::Result<IdentifiedLicense<'a>> {
    Ok(IdentifiedLicense {
        id_from_name: id_from_name(license),
        ids_from_content: ids_from_content(scanner, license)?,
        license,
    })
}

fn ids_from_content(scanner: &Scanner, license: &Local) -> anyhow::Result<Vec<LicenseId>> {
    let scanned = scanner.scan(&std::fs::read_to_string(&license.location)?.into());
    Ok(scanned
        .license
        .iter()
        .chain(scanned.containing.iter().map(|c| &c.license))
        .filter_map(|license| spdx::license_id(license.name))
        .collect())
}

fn id_from_name(license: &Local) -> Option<LicenseId> {
    // slightly arbitrarily preferring earlier words, and more precise names
    license
        .name
        .split('-')
        .flat_map(possible_ids_from_word)
        .next()
}

fn possible_ids_from_word(word: &str) -> impl Iterator<Item = LicenseId> {
    let precise = spdx::license_id(word).into_iter();
    let imprecise = spdx::imprecise_license_id(word).map(|(id, _)| id);
    precise.chain(imprecise)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_identified_licenses() {
        let apache_license_id = spdx::license_id("Apache-2.0").unwrap();
        let license_file = temp_file(include_bytes!("../tests/ahash-LICENSE-APACHE"));
        let licenses = [Local {
            package: "anyhow".to_string(),
            name: "LICENSE-APACHE".to_string(),
            location: license_file.to_path_buf(),
        }];
        assert_eq!(
            identified_licenses(&licenses).unwrap(),
            vec![IdentifiedLicense {
                license: &licenses[0],
                ids_from_content: vec![apache_license_id],
                id_from_name: Some(apache_license_id),
            }]
        );
    }

    #[test]
    fn test_identified_licenses_is_not_pixar_pixar() {
        let apache_license_id = spdx::license_id("Apache-2.0").unwrap();
        let license_file = temp_file(include_bytes!("../tests/anyhow-LICENSE-APACHE"));
        let licenses = [Local {
            package: "anyhow".to_string(),
            name: "LICENSE-APACHE".to_string(),
            location: license_file.to_path_buf(),
        }];
        assert_eq!(
            identified_licenses(&licenses).unwrap(),
            vec![IdentifiedLicense {
                license: &licenses[0],
                ids_from_content: vec![apache_license_id],
                id_from_name: Some(apache_license_id),
            }]
        );
    }

    fn temp_file(contents: &[u8]) -> tempfile::TempPath {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        file.write_all(contents).unwrap();
        file.flush().unwrap();
        file.into_temp_path()
    }
}
