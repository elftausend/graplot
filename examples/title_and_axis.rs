use graplot::{x, Plot};

fn main() {
    let mut plot = Plot::new((|x: f64| x.sin(), x(4.)));
    plot.set_title("sine wave of a function that is great");
    plot.set_title("t");
    plot.show();
}
