use crate::lint::{CombinedReport, Level};
use colored::Colorize;
use std::process::ExitCode;

pub struct Reporter {
    quiet: bool,
    errored: bool,
}

impl Reporter {
    pub fn new(quiet: bool) -> Self {
        Self {
            quiet,
            errored: false,
        }
    }

    pub fn report(&mut self, report: CombinedReport) {
        match report.level {
            Level::Info => self.info(report.to_string()),
            Level::Warning => self.warning(report.to_string()),
            Level::Error => self.error(report.to_string()),
        }
    }

    pub fn info(&self, message: String) {
        if !self.quiet {
            eprintln!("{}: {}", "   info".white().bold(), message);
        }
    }

    pub fn warning(&mut self, message: String) {
        if !self.quiet {
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
