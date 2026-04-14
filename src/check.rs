use crate::Arguments;
use crate::lint::{
    copy_left, extraneous, misnamed, missing_or_unexpected, no_licenses, unknown_type, unmet_spdx,
};
use anyhow::Context;
use std::process::ExitCode;

pub fn check(args: &Arguments) -> anyhow::Result<ExitCode> {
    let filter_rules = crate::filter::FilterRules::new(args);
    let mut reporter = crate::reporter::Reporter::new(args);
    let dependencies: Vec<_> = crate::package::dependencies(args)
        .context("failed to get dependency information")?
        .collect();
    let licenses = crate::local::output_folder_licenses(&args.license_directory);
    let (missing, unexpected) = missing_or_unexpected(&dependencies, &licenses);
    let licenses = crate::identity::identified_licenses(&licenses)?;

    [
        missing,
        unmet_spdx(&dependencies, &licenses),
        copy_left(&licenses),
        no_licenses(&args.license_directory, &dependencies, &licenses),
        unknown_type(&licenses),
        misnamed(&licenses),
        extraneous(&dependencies, &licenses),
        unexpected,
    ]
    .into_iter()
    .filter_map(|x| x)
    .filter_map(|r| filter_rules.filter(r))
    .for_each(|r| reporter.report(r));

    Ok(reporter.exit_code())
}
