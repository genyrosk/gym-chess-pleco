use std::fmt;
use std::fmt::Debug;

use pyo3::exceptions::PyOSError;
use pyo3::prelude::*;

pub struct CustomError(pleco::board::FenBuildError);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<pleco::board::FenBuildError> for CustomError {
    fn from(err: pleco::board::FenBuildError) -> CustomError {
        CustomError(err)
    }
}

impl From<CustomError> for PyErr {
    fn from(err: CustomError) -> PyErr {
        PyOSError::new_err(err.to_string())
    }
}
