use crate::CheckArguments;
use crate::lint::{
    CombineReports, copy_left, extraneous, misnamed, missing_or_unexpected, no_cargo_license,
    no_licenses, unknown_type, unmet_spdx,
};
use std::process::ExitCode;

pub fn check(args: CheckArguments) -> anyhow::Result<ExitCode> {
    let metadata = crate::config::crate_metadata(&args.common.project_directory)?;
    let config = crate::config::config(&metadata)?;
    let args = CheckArguments {
        common: config.common.overwrite_with(args.common),
        filter: config.filter.overwrite_with(args.filter),
    };
    let filter_rules = crate::filter::FilterRules::new(&args.filter);
    let mut reporter = crate::reporter::Reporter::new(args.common.quiet);
    let dependencies: Vec<_> = crate::package::dependencies(&args.common, &metadata).collect();
    let licenses = crate::license::output_folder_licenses(&args.common.license_directory);
    let (missing, unexpected) = missing_or_unexpected(&dependencies, &licenses);
    let licenses = crate::identity::identified_licenses(&licenses)?;

    let mut reports: Vec<_> = missing
        .into_iter()
        .chain(no_cargo_license(&crate::package::root_package(&metadata)))
        .chain(unmet_spdx(&dependencies, &licenses))
        .chain(copy_left(&licenses))
        .chain(no_licenses(
            &args.common.license_directory,
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
