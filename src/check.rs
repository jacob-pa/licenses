use crate::CheckArguments;
use crate::lint::{
    CombineReports, copy_left, extraneous, misnamed, missing_or_unexpected, no_licenses,
    unknown_type, unmet_spdx,
};
use anyhow::Context;
use std::process::ExitCode;

pub fn check(args: &CheckArguments) -> anyhow::Result<ExitCode> {
    let filter_rules = crate::filter::FilterRules::new(args);
    let mut reporter = crate::reporter::Reporter::new(args.common.quiet);
    let dependencies: Vec<_> = crate::package::dependencies(&args.common)
        .context("failed to get dependency information")?
        .collect();
    let licenses = crate::local::output_folder_licenses(&args.common.license_directory);
    let (missing, unexpected) = missing_or_unexpected(&dependencies, &licenses);
    let licenses = crate::identity::identified_licenses(&licenses)?;

    missing
        .into_iter()
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
        .for_each(|r| reporter.report(r));

    Ok(reporter.exit_code())
}
