use log::{trace, LevelFilter};

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

}
