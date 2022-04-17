use graplot::{x, Plot};

fn main() {
    let plot = Plot::new((|x: f64| x.sin(), x(4.)));
    /*
    let mut xs = [0.; 1000];

    let mut add = 0f64;
    for idx in 0..1000 {
        xs[idx] = add/1000.;
        add += 1.;
    }

    let mut ys = [0.; 1000];
    for (i, y) in ys.iter_mut().enumerate() {
        *y = (2. * std::f64::consts::PI * xs[i]).sin();
    }

    let plot = Plot::new((xs, ys));
    */
    plot.show();
}
