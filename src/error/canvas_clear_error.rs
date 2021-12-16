use std::fmt;
use crate::plotting::DrawingAreaError;
use plotters_canvas::CanvasBackend;

use super::canvas_get_error::CanvasGetError;

// See https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsError.html
#[derive(Debug)]
pub enum CanvasClearError {
    DrawingAreaError(String),
    SerdeError,
    CanvasGetError
}
impl std::error::Error for CanvasClearError {}
impl fmt::Display for CanvasClearError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Canvas couldn't be cleared.")
    }
}

impl From<DrawingAreaError<CanvasBackend>> for CanvasClearError {
    fn from(err: DrawingAreaError<CanvasBackend>) -> Self {
        Self::DrawingAreaError(err.to_string())
    }
}

impl From<serde_json::Error> for CanvasClearError {
    fn from(_err: serde_json::Error) -> Self {
        Self::SerdeError
    }
}

impl From<CanvasGetError> for CanvasClearError {
    fn from(_err: CanvasGetError) -> Self {
        Self::CanvasGetError
    }
}