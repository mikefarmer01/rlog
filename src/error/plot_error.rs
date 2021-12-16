use std::fmt;

use super::canvas_get_error::CanvasGetError;

#[derive(Debug)]
pub struct PlotError{}

impl std::error::Error for PlotError {}
impl fmt::Display for PlotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error on plotting.")
    }
}

impl From<CanvasGetError> for PlotError {
    fn from(_err: CanvasGetError) -> Self {
        Self{}
    }
}