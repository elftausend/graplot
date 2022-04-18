use graplot::{x, Plot};

fn main() {
    let mut plot = Plot::new((|x: f64| x.cos(), x(6.)));
    
    plot.set_title("cosine wave");
    plot.set_xlabel("x axis");
    plot.set_ylabel("y axis");
    plot.show();
}
