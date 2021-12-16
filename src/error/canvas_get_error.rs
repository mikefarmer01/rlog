use std::fmt;

// See https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsError.html
#[derive(Debug)]
pub enum CanvasGetError {
    HtmlWindowError,
    HtmlDocumentError,
    HtmlElementError,
    HtmlCanvasElementError,
    CanvasBackendError,
}
impl std::error::Error for CanvasGetError {}
impl fmt::Display for CanvasGetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Couldn't get canvas.")
    }
}