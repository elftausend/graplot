use graplot::Plot;

fn main() {
    let plot = Plot::new((|x: f64| x.powf(2.) + 0.5, "r-"));
    plot.show();

    //----------------------

    // c ... cyan, - ... solid line, o ... ring marker
    let plot = Plot::new(([-4., -3., -3.4, -3.75, -4.1], "c-o"));
    plot.show();

    //----------------------

    let mut xs = [0.; 20000]; 
    
    let mut add = -10000f64;
    for idx in 0..20000 {
        xs[idx] = add/1000.;
        add += 1.;
    }
     
    let mut ys = [0.; 20000];
    for (i, y) in ys.iter_mut().enumerate() {
        *y = xs[i].powf(2.);
    }

    let plot = Plot::new((xs, ys, "-y"));
    plot.show();
}