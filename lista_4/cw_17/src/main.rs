use plotly::{
    color::NamedColor,
    common::{AxisSide, Font, Line, Mode, Title},
    layout::{Axis, Layout, Shape, ShapeLine, ShapeType},
    Histogram, ImageFormat, Plot, Scatter,
};
use rand::{distributions::Uniform, Rng};

fn main() {
    let n_stations_list = [
        500, 600, 700, 800, 900, 1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000,
    ];
    let trials = 1000;

    for n_stations in n_stations_list {
        let (results, mean_estimate, deviation_estimate) = approximate_counting(n_stations, trials);

        let actual_count = n_stations;
        let error = (mean_estimate - actual_count as f64) / actual_count as f64 * 100.0;

        let trace = Histogram::new(results)
            .name("Estimated Count")
            .opacity(0.7)
            .n_bins_x(30);

        let mut plot = Plot::new();
        plot.add_trace(trace);

        let mut layout = Layout::new()
            .title(Title::new(&format!(
                "Approximate Counting of {n_stations} Stations\n
            Mean Estimate: {mean_estimate:.2}, Dev: {deviation_estimate:.2},
            Error: {error:.2}%"
            )))
            .x_axis(Axis::new().title(Title::new("Estimated Number of Stations")))
            .y_axis(Axis::new().title(Title::new("Frequency")));

        layout.add_shape(
            Shape::new()
                .shape_type(ShapeType::Line)
                .y_ref("paper")
                .x_ref("x")
                .x0(actual_count)
                .x1(actual_count)
                .y0(0)
                .y1(1)
                .line(
                    ShapeLine::new()
                        .color(NamedColor::Red)
                        .width(2.0)
                        .dash(plotly::common::DashType::DashDot),
                ),
        );

        plot.set_layout(layout);

        plot.write_image(
            format!("stations_{n_stations}.pdf"),
            ImageFormat::PDF,
            1280,
            900,
            1.0,
        );
    }

    let n_stations_range = (400..=10000).step_by(100);
    let mut errors = vec![];
    let mut deviance = vec![];

    for n_stations in n_stations_range.clone() {
        let (_, mean_estimate, deviation_estimate) = approximate_counting(n_stations, trials);
        let actual_count = n_stations;
        let error = (mean_estimate - actual_count as f64) / actual_count as f64 * 100.0;
        errors.push(error);
        deviance.push(deviation_estimate);
    }

    let trace1 = Scatter::new(n_stations_range.clone().into_iter().collect(), errors)
        .mode(Mode::LinesMarkers)
        .line(Line::new().color(NamedColor::Blue));

    let trace2 = Scatter::new(n_stations_range.into_iter().collect(), deviance)
        .mode(Mode::LinesMarkers)
        .line(Line::new().color(NamedColor::Red))
        .y_axis("y2");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let layout = Layout::new()
        .title(Title::new(
            "Error Percentage and Standard Deviation of Approximate Counting vs Number of Stations",
        ))
        .x_axis(Axis::new().title(Title::new("Number of Stations")))
        .y_axis(
            Axis::new()
                .title(Title::new("Error Percentage").font(Font::new().color(NamedColor::Blue))),
        )
        .y_axis2(
            Axis::new()
                .title(Title::new("Standard Deviation").font(Font::new().color(NamedColor::Red)))
                .overlaying("y")
                .side(AxisSide::Right),
        )
        .show_legend(false);

    plot.set_layout(layout);

    plot.write_image(
        format!("error_and_dev_chart.pdf"),
        ImageFormat::PDF,
        1280,
        900,
        1.0,
    );
}

fn approximate_counting(n_stations: usize, trials: usize) -> (Vec<f64>, f64, f64) {
    let mut results: Vec<f64> = vec![];

    for _ in 0..trials {
        let rng = rand::thread_rng();
        let unif = Uniform::from(0.0..1.0);
        let mut zeta_values = rng
            .sample_iter(&unif)
            .take(n_stations)
            .collect::<Vec<f64>>();
        zeta_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if n_stations <= 400 {
            results.push(n_stations as f64);
        } else {
            results.push(399.0 / zeta_values[399]);
        }
    }

    let mean_estimate = results.iter().sum::<f64>() / trials as f64;

    let variance = results
        .iter()
        .map(|value| {
            let diff = mean_estimate - (*value as f64);
            diff * diff
        })
        .sum::<f64>()
        / trials as f64;

    let deviation_estimate = variance.sqrt();
    return (results, mean_estimate, deviation_estimate);
}
