mod error_code;
mod monprof_error;

use log::error;
use std::iter;

pub use self::monprof_error::MonprofError;

pub fn print_error_chain(error: &dyn MonprofError) {
    error!("ERROR: {}", error);

    if let Some(source) = error.source() {
        error!("");
        error!("Caused by:");

        for (error_number, error) in
            iter::successors(Some(source), |&error| error.source()).enumerate()
        {
            error!("\t{}: {}", error_number, error);
        }
    }
}
