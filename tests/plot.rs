use graplot::Plot;

#[test]
fn test_every_quadrant() {
    let plot = Plot::new(([-1., -1.5, -2.1, -1., -1.5, -2.1, 1., 1.5, 2.1, 1., 1.5, 2.1], [-2., -1., -2.5, 2., 1., 2.5, -2., -1., -2.5, 2., 1., 2.5], "ro"));
    plot.show();
}