use graplot::Plot;

fn main() {
    let plot = Plot::new(|x: f64| x.powf(2.));
    plot.show();
}
