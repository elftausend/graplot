use graplot::Plot;


#[test]
fn markers() {
    let xs = [1., 8.];
    let ys = [3., 10.,];

    let plot = Plot::new((xs, ys, "-o"));
    plot.show();
}