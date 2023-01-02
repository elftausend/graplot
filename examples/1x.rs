use graplot::Plot;

fn main() {
    let plot = Plot::new(|x| 1. / (x+4.));
    plot.show();
}