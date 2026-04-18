use crate::Lint;
use crate::lint::{Level, Report};
use crate::local::Local;
use crate::package::Package;
use std::collections::HashSet;

pub fn missing_or_unexpected(
    dependencies: &[Package],
    licenses: &[Local],
) -> (Vec<Report>, Vec<Report>) {
    let expected: HashSet<_> = dependencies.iter().map(|p| p.id()).collect();
    let found: HashSet<_> = licenses.iter().map(|l| l.package_id()).collect();

    let missing = expected
        .difference(&found)
        .cloned()
        .map(|item| Report {
            lint: Lint::Missing,
            level: Level::Error,
            item,
        })
        .collect();

    let unexpected = found
        .difference(&expected)
        .flat_map(|id| {
            licenses
                .iter()
                .filter(|l| l.package_id() == *id)
                .map(|l| l.location_file_name())
        })
        .map(|item| Report {
            lint: Lint::Unexpected,
            level: Level::Info,
            item,
        })
        .collect();

    (missing, unexpected)
}
