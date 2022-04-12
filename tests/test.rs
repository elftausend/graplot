use graplot::Plot;


#[test]
fn test_plot() {
    let plot = Plot::new(([-4., -2., 1., 4.], "o"));
    plot.show();
}