use super::error_code::ErrorCode;
use std::error::Error;

pub trait MonprofError: Error + ErrorCode {}
