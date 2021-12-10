use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

pub fn plot(canvas_id: &str, periods_demands: &Vec<f32>) {
    let elem_canvas = get_canvas(canvas_id);
    let canvas_backend = CanvasBackend::with_canvas_object(elem_canvas).unwrap();
    let root = canvas_backend.into_drawing_area();
    root.fill(&WHITE).unwrap();

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
        .draw_series(LineSeries::new(line_data.into_iter(), &BLUE))
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
