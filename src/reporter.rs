use crate::Arguments;
use crate::lint::{Level, Report};
use colored::Colorize;
use std::process::ExitCode;

pub struct Reporter {
    quiet: bool,
    error_on_warning: bool,
    errored: bool,
}

impl Reporter {
    pub fn new(args: &Arguments) -> Self {
        Self {
            quiet: args.quiet,
            error_on_warning: args.error_on_warning,
            errored: false,
        }
    }

    pub fn report(&mut self, report: Option<Report>) {
        if let Some(report) = report {
            match report.level {
                Level::Info => self.info(report.message),
                Level::Warning => self.warning(report.message),
                Level::Error => self.error(report.message),
            }
        }
    }

    pub fn info(&self, message: String) {
        if !self.quiet {
            eprintln!("{}: {}", "   info".white().bold(), message);
        }
    }

    pub fn warning(&mut self, message: String) {
        if self.error_on_warning {
            self.error(message);
        } else if !self.quiet {
            eprintln!("{}: {}", "warning".yellow().bold(), message);
        }
    }

    pub fn error(&mut self, message: String) {
        self.errored |= true;
        if !self.quiet {
            eprintln!("{}: {}", "  error".red().bold(), message);
        }
    }

    pub fn exit_code(&self) -> ExitCode {
        if self.errored {
            ExitCode::FAILURE
        } else {
            ExitCode::SUCCESS
        }
    }
}
