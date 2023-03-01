use pyo3::prelude::*;

#[pyclass]
#[derive(Copy, Clone, Debug)]
pub struct Score(pleco::core::score::Score);

#[pymethods]
impl Score {
    fn __repr__(&self) -> String {
        self.0.to_string()
    }
}

impl From<pleco::core::score::Score> for Score {
    fn from(score: pleco::core::score::Score) -> Score {
        Score(score)
    }
}

impl From<Score> for pleco::core::score::Score {
    fn from(score: Score) -> pleco::core::score::Score {
        score.0
    }
}
