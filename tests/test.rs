use graplot::Plot;


#[test]
fn test_plot() {
    let plot = Plot::new(([-4., -2., 1., 4.], "o"));
    plot.show();
}

#[test]
fn test_plot2() {
    let plot = Plot::new([-4., -2., 1., 4.]);
    plot.show();
}

#[test]
fn test_sine_wave() {
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
    plot.show();
}

#[test]
fn test_x_squared() {
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

    let plot = Plot::new((xs, ys));
    plot.show();
}

#[test]
fn test_fn() {
    //let plot = Plot::new((|x: f64| (2. * std::f64::consts::PI * x).sin(), 15000));
    let plot = Plot::new((|x: f64| x.powf(2.), 1000));
    plot.show();
}