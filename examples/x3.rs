use graplot::{x, Plot};

fn main() {
    // x(...) ... sets the absolute max value for x
    let plot = Plot::new((|x: f64| x.powf(3.) + x.powf(2.) - 0.08, x(1.)));
    plot.show();
}
