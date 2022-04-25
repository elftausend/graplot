use graplot::{polynomial, x, Plot, PlotArg, Polynomial};

#[test]
fn test_poly_2nd() {
    let poly = Polynomial::new(&[2., 3., 1.], &[2., 3., 2.]);
    let plot = Plot::new((poly, x(10.)));
    plot.show();
}

#[test]
fn test_poly_1st() {
    let poly = polynomial(&[2., 5.], &[2., 9.]);
    (poly, x(1000.)).as_plot().show();
}
