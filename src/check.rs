use crate::Arguments;
use crate::lint::{
    copy_left, extraneous, misnamed, missing_or_unexpected, no_licenses, unknown_type, unmet_spdx,
};
use anyhow::Context;
use std::process::ExitCode;

pub fn check(args: &Arguments) -> anyhow::Result<ExitCode> {
    let mut reporter = crate::reporter::Reporter::new(args);
    let dependencies: Vec<_> = crate::package::dependencies(args)
        .context("failed to get dependency information")?
        .collect();
    let licenses = crate::local::output_folder_licenses(&args.license_directory);
    let (missing, unexpected) = missing_or_unexpected(&dependencies, &licenses);
    let licenses = crate::identity::identified_licenses(&licenses)?;

    reporter.report(missing);
    reporter.report(unmet_spdx(&dependencies, &licenses));
    reporter.report(copy_left(&licenses));
    reporter.report(no_licenses(
        &args.license_directory,
        &dependencies,
        &licenses,
    ));
    reporter.report(unknown_type(&licenses));
    reporter.report(misnamed(&licenses));
    reporter.report(extraneous(&dependencies, &licenses));
    reporter.report(unexpected);

    Ok(reporter.exit_code())
}
