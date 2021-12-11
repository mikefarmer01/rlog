use plotters::{prelude::*, coord::Shift};
use plotters_canvas::CanvasBackend;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

use crate::utils::str_to_rgb;

pub fn plot(canvas_id: &str, periods_demands: &Vec<f32>, color: RGBColor) {
    let root = get_root(canvas_id);

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
}

fn get_canvas(canvas_id: &str) -> HtmlCanvasElement {
    let window = web_sys::window().expect("No global `window` exists.");
    let document = window.document().expect("No document exists on window.");
    let element = document.get_element_by_id(&canvas_id).expect(&format!(
        "Html canvas with name {} doesn't exist.",
        &canvas_id
    ));
    element
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect(&format!(
            "Html element with name {} is no canvas.",
            &canvas_id
        ))
}

fn get_root(canvas_id: &str) -> DrawingArea<CanvasBackend, Shift> {
    let elem_canvas = get_canvas(canvas_id);
    let canvas_backend = CanvasBackend::with_canvas_object(elem_canvas).unwrap();
    canvas_backend.into_drawing_area()
}

pub fn clear(canvas_id: &str) {
    let root = get_root(canvas_id);
    let bg_color = get_bg_color();
    root.fill(&bg_color).unwrap();
}

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