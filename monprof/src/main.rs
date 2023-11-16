mod cli;

use clap::Parser;
use cli::Arguments;
use log::{trace, LevelFilter};
use monproflib::{
    error::{self, MonprofError},
    monitor::Monitor,
};
use std::{path::PathBuf, process::ExitCode};

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
    let args = Arguments::parse();
    init_logging(args.verbose, args.quiet, args.json_output);

    let execution_result: Result<(), Box<dyn MonprofError>> = {
        let monitor: Monitor = Monitor::from_sysfs(PathBuf::from(
            "/sys/devices/pci0000:00/0000:00:01.0/0000:01:00.0/drm/card0/card0-HDMI-A-1",
        ));
        Ok(())
    };

    if let Err(error) = execution_result {
        report_error(error)
    } else {
        ExitCode::SUCCESS
    }
}
