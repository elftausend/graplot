use graplot::{Plot, x};

fn main() {
    let mut plot = Plot::new((|x: f64| (3f64).powf(x), x(3.)));
    let plot_2x = Plot::new((|x: f64| (2f64).powf(x), x(4.), "r"));
    plot.add(plot_2x);
    let plot_1_2x = Plot::new((|x: f64| (1./2f64).powf(x), x(4.), "b"));
    plot.add(plot_1_2x);
    plot.show();
}