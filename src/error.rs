use std::fmt;
use std::fmt::Debug;

use pyo3::exceptions::PyOSError;
use pyo3::prelude::*;

#[derive(Debug, Clone)]
pub struct CustomError {
    msg: String,
}

impl CustomError {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CustomError: {}", self.msg)
    }
}

pub struct FenBuildError(pleco::board::FenBuildError);

impl fmt::Display for FenBuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<pleco::board::FenBuildError> for FenBuildError {
    fn from(err: pleco::board::FenBuildError) -> FenBuildError {
        FenBuildError(err)
    }
}

impl From<FenBuildError> for PyErr {
    fn from(err: FenBuildError) -> PyErr {
        PyOSError::new_err(err.to_string())
    }
}
