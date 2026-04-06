use std::process::ExitCode;

pub trait Reporter {
    fn info(&self, message: String);
    fn warning(&mut self, message: String);
    fn error(&mut self, message: String);
}

pub struct ConfiguredReporter<T> {
    reporter: T,
    silent: bool,
    warnings_as_errors: bool,
    errored: bool,
}

impl<T> ConfiguredReporter<T> {
    pub fn new(reporter: T, silent: bool, warnings_as_errors: bool) -> Self {
        Self {
            reporter,
            silent,
            warnings_as_errors,
            errored: false,
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

impl<T: Reporter> Reporter for ConfiguredReporter<T> {
    fn info(&self, message: String) {
        if !self.silent {
            self.reporter.info(message);
        }
    }

    fn warning(&mut self, message: String) {
        if self.warnings_as_errors {
            self.errored |= true;
        }
        if !self.silent {
            self.reporter.warning(message);
        }
    }

    fn error(&mut self, message: String) {
        self.errored |= true;
        if !self.silent {
            self.reporter.error(message);
        }
    }
}

pub struct StderrReporter;

impl Reporter for StderrReporter {
    fn info(&self, message: String) {
        eprintln!("{}", message);
    }

    fn warning(&mut self, message: String) {
        eprintln!("warning: {}", message);
    }

    fn error(&mut self, message: String) {
        eprintln!("error: {}", message);
    }
}
