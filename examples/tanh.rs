use graplot::{x, Plot};

fn main() {
    let plot = Plot::new((|x: f64| x.tanh(), x(6.)));
    plot.show()
}
