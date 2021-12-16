use plotters::drawing::DrawingAreaErrorKind;
use plotters::{coord::Shift, prelude::*};
use plotters_canvas::CanvasBackend;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

use crate::error::canvas_clear_error::CanvasClearError;
use crate::error::canvas_get_error::CanvasGetError;
use crate::error::canvas_get_error::CanvasGetError::*;
use crate::error::plot_error::PlotError;
use crate::utils::str_to_rgb;

pub fn plot(canvas_id: &str, periods_demands: &Vec<f32>, color: RGBColor) -> Result<(), PlotError> {
    let root = get_root(canvas_id)?;

    let periods_range = 0..periods_demands.len();
    let line_data: Vec<(usize, f32)> = periods_demands
        .into_iter()
        .zip(periods_range.clone())
        .map(|(a, b)| (b, *a))
        .collect();

    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(periods_range, 22f32..38f32)
        .unwrap();

    chart
        .draw_series(LineSeries::new(line_data.into_iter(), &color))
        .unwrap();
    Ok(())
}

fn get_canvas(canvas_id: &str) -> Result<HtmlCanvasElement, CanvasGetError> {
    let window = web_sys::window().ok_or(HtmlWindowError)?;
    let document = window.document().ok_or(HtmlDocumentError)?;
    let element = document
        .get_element_by_id(&canvas_id)
        .ok_or(CanvasGetError::HtmlElementError)?;
    let canvas = match element.dyn_into::<HtmlCanvasElement>() {
        Ok(canvas) => Ok(canvas),
        Err(_) => Err(HtmlCanvasElementError),
    };
    let canvas = canvas?;
    Ok(canvas)
}

fn get_root(canvas_id: &str) -> Result<DrawingArea<CanvasBackend, Shift>, CanvasGetError> {
    let elem_canvas = get_canvas(canvas_id)?;
    let canvas_backend =
        CanvasBackend::with_canvas_object(elem_canvas).ok_or(CanvasBackendError)?;
    Ok(canvas_backend.into_drawing_area())
}

pub fn clear(canvas_id: &str) -> Result<(), CanvasClearError> {
    let root = get_root(canvas_id)?;
    let bg_color = get_bg_color();
    root.fill(&bg_color)?;
    Ok(())
}

//TODO: Proper error propagation
fn get_bg_color() -> RGBColor {
    let window = web_sys::window().expect("No global `window` exists.");
    let document = window.document().expect("No document exists on window.");
    let body = document.body().expect("No body exists for document.");

    let styles = window.get_computed_style(&body).unwrap().unwrap();
    let bg_color = styles.get_property_value("background-color");
    let bg_color: String = match bg_color {
        Ok(bg_color) => bg_color,
        Err(_) => "rgb(0, 0, 0)".to_owned(),
    };
    str_to_rgb(bg_color)
}
pub type DrawingAreaError<T> = DrawingAreaErrorKind<<T as DrawingBackend>::ErrorType>;
