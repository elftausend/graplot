use graplot::{x, Plot};

fn main() {
    let plot = Plot::new((|x: f64| 1. / (1. + (-x).exp()), x(6.)));
    plot.show()
}
