use graplot::{x, Plot, Polynomial};

fn main() {
    let poly = Polynomial::new(&[2., 3., 1.], &[2., 3., 2.]);
    let plot = Plot::new((poly, x(10.)));
    plot.show();
}
