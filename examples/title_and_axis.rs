use graplot::{x, Plot};

fn main() {
    let mut plot = Plot::new((|x: f64| x.sin(), x(4.)));
    plot.set_title("aaaaaaaa");
    plot.show();
}
