use log::{trace, LevelFilter};
use monproflib::error::{self, MonprofError};
use std::process::ExitCode;

fn report_error(error: Box<dyn MonprofError>) -> ExitCode {
    error::print_error_chain(error.as_ref());
    error.error_code()
}

fn init_logging(verbose: bool, quiet: bool, json_output: bool) {
    let mut level = LevelFilter::Info;

    if quiet || json_output {
        level = LevelFilter::Off;
    } else if verbose {
        level = LevelFilter::Trace;
    }

    env_logger::builder().filter(None, level).init();
    trace!("Logging system initialized");
}

fn main() -> ExitCode {
    let execution_result: Result<(), Box<dyn MonprofError>> = { Ok(()) };

    if let Err(error) = execution_result {
        report_error(error)
    } else {
        ExitCode::SUCCESS
    }
}
