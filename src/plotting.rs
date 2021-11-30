use plotters::prelude::*;
use plotters_canvas::CanvasBackend;

pub fn plot(line_data: Vec<(i32, f32)>) {
    let canvas_backend = CanvasBackend::new("demandPlot").expect("Cannot find Canvas.");
    let root = canvas_backend.into_drawing_area();
    root.fill(&WHITE).unwrap();
    
    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(0..100, 22f32..38f32).unwrap();
    
    chart.draw_series(LineSeries::new(line_data.into_iter(), &BLUE)).unwrap();
}
