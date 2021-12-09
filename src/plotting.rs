use plotters::{prelude::*, coord::Shift};
use plotters_canvas::CanvasBackend;

fn init() -> DrawingArea<CanvasBackend, Shift> {
    let canvas_backend = CanvasBackend::new("demandPlot").expect("Cannot find Canvas.");
    let root = canvas_backend.into_drawing_area();
    root.fill(&WHITE).unwrap();
    return root
}

pub fn plot(periods_demands: &Vec<f32>) {
    let root = init();
    let periods_range = 0..periods_demands.len();

    let line_data: Vec<(usize, f32)> = periods_demands
        .into_iter()
        .zip(periods_range.clone())
        .map(|(a, b)| (b, *a)).collect();

    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(periods_range, 22f32..38f32)
        .unwrap();

    chart
        .draw_series(LineSeries::new(line_data.into_iter(), &BLUE))
        .unwrap();
}
