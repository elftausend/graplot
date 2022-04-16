use graplot::{Plot, x};

#[test]
fn test_xend_squared() {
    let plot = Plot::new((|x: f64| x.powi(2), x(400000.)));
    
    plot.show();
}


#[test]
fn test_xend_poly3() {
    let plot = Plot::new((|x: f64| x.powi(3) + x.powi(2), x(1.6), "-"));
    plot.show();
}