use crate::lint::report::ReportIfAny;
use crate::lint::{Level, Report};
use crate::local::Local;
use crate::package::Package;
use std::collections::HashSet;

pub fn missing_or_unexpected(
    dependencies: &[Package],
    licenses: &[Local],
) -> (Option<Report>, Option<Report>) {
    let expected: HashSet<_> = dependencies.iter().map(|p| p.name.clone()).collect();
    let found: HashSet<_> = licenses.iter().map(|l| l.package.clone()).collect();

    let missing = expected.difference(&found).cloned().report_if_any(
        Level::Error,
        "dependencies without any licenses",
        |s| s.to_string(),
    );

    let unexpected = found
        .difference(&expected)
        .flat_map(|p| {
            licenses
                .iter()
                .filter(|l| l.package == *p)
                .map(|l| l.file_name())
        })
        .report_if_any(
            Level::Info,
            "license files from packages that are not dependencies",
            |s| s,
        );

    (missing, unexpected)
}
