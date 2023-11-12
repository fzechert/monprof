use std::process::ExitCode;

pub trait ErrorCode {
    fn error_code(&self) -> ExitCode;
}
