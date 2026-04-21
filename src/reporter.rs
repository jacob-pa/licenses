use crate::lint::{CombinedReport, Level};
use colored::Colorize;
use std::process::ExitCode;

pub trait Reporter {
    fn report(&mut self, report: CombinedReport);
    fn info(&self, message: String);
    fn warning(&mut self, message: String);
    fn error(&mut self, message: String);
    fn exit_code(&self) -> ExitCode;
}

pub struct StdoutReporter {
    quiet: bool,
    errored: bool,
}

impl StdoutReporter {
    pub fn new(quiet: bool) -> Self {
        Self {
            quiet,
            errored: false,
        }
    }
}

impl Reporter for StdoutReporter {
    fn report(&mut self, report: CombinedReport) {
        match report.level {
            Level::Info => self.info(report.to_string()),
            Level::Warning => self.warning(report.to_string()),
            Level::Error => self.error(report.to_string()),
        }
    }

    fn info(&self, message: String) {
        if !self.quiet {
            eprintln!("{}: {}", "   info".white().bold(), message);
        }
    }

    fn warning(&mut self, message: String) {
        if !self.quiet {
            eprintln!("{}: {}", "warning".yellow().bold(), message);
        }
    }

    fn error(&mut self, message: String) {
        self.errored |= true;
        if !self.quiet {
            eprintln!("{}: {}", "  error".red().bold(), message);
        }
    }

    fn exit_code(&self) -> ExitCode {
        if self.errored {
            ExitCode::FAILURE
        } else {
            ExitCode::SUCCESS
        }
    }
}
