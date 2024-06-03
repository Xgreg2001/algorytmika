use plotly::{
    common::Title,
    layout::{Axis, GridPattern, LayoutGrid},
    Bar, ImageFormat, Layout, Plot,
};
use rand::Rng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let m = 10_usize.pow(6);
    let n = 10_usize.pow(3);

    let mut rng = rand::thread_rng();

    let mut buckets_uniform = vec![0; n];
    for _ in 0..m {
        let bucket = rng.gen_range(0..n);
        buckets_uniform[bucket] += 1;
    }

    let mut buckets_two_choices = vec![0; n];
    for _ in 0..m {
        let bucket1 = rng.gen_range(0..n);
        let bucket2 = rng.gen_range(0..n);
        if buckets_two_choices[bucket1] <= buckets_two_choices[bucket2] {
            buckets_two_choices[bucket1] += 1;
        } else {
            buckets_two_choices[bucket2] += 1;
        }
    }

    let x = (0..n).collect::<Vec<_>>();

    let trace1 = Bar::new(x.clone(), buckets_uniform).name("Uniform");
    let trace2 = Bar::new(x, buckets_two_choices)
        .name("Two Choices")
        .x_axis("x2")
        .y_axis("y2");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let layout = Layout::new()
        .grid(
            LayoutGrid::new()
                .rows(1)
                .columns(2)
                .pattern(GridPattern::Independent),
        )
        .title(Title::new("Two Choices algorithm comparison"))
        .y_axis(Axis::new().range(vec![900, 1100]))
        .y_axis2(Axis::new().range(vec![900, 1100]));

    plot.set_layout(layout);

    plot.write_image("plot.pdf", ImageFormat::PDF, 1280, 900, 1.0);

    Ok(())
}
