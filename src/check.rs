use crate::config::CheckConfig;
use crate::lint::{
    CombineReports, copy_left, extraneous, misnamed, missing_or_unexpected, no_cargo_license,
    no_licenses, unknown_type, unmet_spdx,
};
use crate::metadata::Metadata;
use crate::reporter::Reporter;
use std::process::ExitCode;

pub fn check(
    metadata: impl Metadata,
    config: CheckConfig,
    mut reporter: impl Reporter,
) -> anyhow::Result<ExitCode> {
    let filter_rules = crate::filter::FilterRules::new(&config);
    let dependencies: Vec<_> = crate::package::dependencies(&config.common, &metadata).collect();
    let licenses = crate::license::output_folder_licenses(&config.common.license_directory);
    let (missing, unexpected) = missing_or_unexpected(&dependencies, &licenses);
    let licenses = crate::identity::identified_licenses(&licenses)?;

    let mut reports: Vec<_> = missing
        .into_iter()
        .chain(no_cargo_license(&crate::package::root_package(&metadata)))
        .chain(unmet_spdx(&dependencies, &licenses))
        .chain(copy_left(&licenses))
        .chain(no_licenses(
            &config.common.license_directory,
            &dependencies,
            &licenses,
        ))
        .chain(unknown_type(&licenses))
        .chain(misnamed(&licenses))
        .chain(extraneous(&dependencies, &licenses))
        .chain(unexpected)
        .filter_map(|r| filter_rules.filter(r))
        .combine_reports()
        .collect();

    reports.sort_by_key(|r| (r.level, r.lint));

    reports.into_iter().for_each(|r| reporter.report(r));

    Ok(reporter.exit_code())
}
