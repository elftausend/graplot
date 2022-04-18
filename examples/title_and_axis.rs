use graplot::{x, Plot};

fn main() {
    let mut plot = Plot::new((|x: f64| x.sin(), x(4.)));
    plot.set_title("sine wave");
    plot.set_ylabel("y axis");
    plot.set_xlabel("y axis");
    plot.show();
}
