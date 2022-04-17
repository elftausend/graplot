use graplot::Plot;

fn main() {
    let xs = [1., 2., 3.];
    let ys = [1.7, 3., 1.9];

    let ys1 = [1.4, 1.6, 1.5];

    let ys2 = [0.9, 1.2, 1.7, 1.9, 2.];

    let mut plot = Plot::new((xs, ys));
    plot.add((xs, ys1, "c-o"));
    plot.add((ys2, "r-"));
    plot.show();
}
