use crate::Lint;
use crate::identity::IdentifiedLicense;
use crate::lint::report::ReportIfAny;
use crate::lint::{Level, Report};
use crate::package::Package;

pub fn unmet_spdx(dependencies: &[Package], licenses: &[IdentifiedLicense]) -> Option<Report> {
    dependencies
        .iter()
        .filter_map(|package| {
            package
                .spdx_license
                .as_ref()
                .map(|expression| (package, expression))
        })
        .filter(|(package, expression)| !spdx_requirements_met(&package.name, expression, licenses))
        .map(|(package, expression)| format!("{} ({})", package.name, expression))
        .report_if_any(Lint::UnmetSpdx, Level::Error)
}

fn spdx_requirements_met(
    package: &str,
    expression: &spdx::Expression,
    licenses: &[IdentifiedLicense],
) -> bool {
    expression.evaluate(|requirement| match requirement.license.id() {
        Some(id) => licenses
            .iter()
            .any(|l| l.license.package == package && l.ids().any(|l| *l == id)),
        None => false,
    })
}
