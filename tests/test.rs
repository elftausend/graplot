use graplot::Plot;

#[test]
fn test_plot() {
    let plot = Plot::new(([-4., -2., 1., 4.], "o-"));
    plot.show();
}

#[test]
fn test_plot2() {
    let plot = Plot::new([-4., -2., 1., 4.]);
    plot.show();
}

#[test]
fn test_plot3() {
    let plot = Plot::new(([-1.6], "o"));
    plot.show();
}

#[test]
fn test_plot4() {
    let xs = [-1., -2., -3., -4.];
    let ys = [-1., -1.5, -1., -1.5];

    let plot = Plot::new((xs, ys));
    plot.show();
}

#[test]
fn test_plot5() {
    let plot = Plot::new(([-1.6, -1.9], "7o-"));
    plot.show();
}

#[test]
fn test_sine_wave() {
    let mut xs = [0.; 1000];

    let mut add = 0f64;
    for idx in 0..1000 {
        xs[idx] = add / 1000.;
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
        xs[idx] = add / 1000.;
        add += 1.;
    }

    let mut ys = [0.; 20000];
    for (i, y) in ys.iter_mut().enumerate() {
        *y = xs[i].powf(2.);
    }

    let plot = Plot::new((xs, ys));
    plot.show();
}

#[cfg(not(target_os="linux"))]
#[test]
fn test_fn() {
    //let plot = Plot::new((|x: f64| (2. * std::f64::consts::PI * x).sin(), 15000));
    //let plot = Plot::new((|x: f64| x.powf(2.), 1000000));
    let plot = Plot::new((|x: f64| x.powf(3.) + x.powf(2.) - 0.08, 10000));
    //plot.show_threaded();
    plot.show();

    let plot = Plot::new((|x: f64| x.powf(2.) + 0.08, 10000));
    plot.show()
}

#[cfg(target_os="linux")]
#[test]
fn test_fn() {
    let plot = Plot::new(|x: f64| x.powf(3.) + x.powf(2.) - 0.08);
    let h = plot.show_threaded();
    
    let plot = Plot::new(|x: f64| x.powf(2.) + 0.08);
    plot.show();
    h.join().unwrap()
}

#[test]
fn test_closure() {
    let plot = Plot::new(|x: f64| x.powf(2.) - 0.5);
    plot.show();
}

#[test]
fn test_poly_3() {
    let mut xs = [0.; 30001];

    let mut add = -15000f64;
    for idx in 0..=30000 {
        xs[idx] = add / 10000.;
        add += 1.;
    }

    let mut ys = [0.; 30001];
    for (i, y) in ys.iter_mut().enumerate() {
        *y = xs[i].powf(3.) + xs[i].powf(2.);
    }
    let plot = Plot::new((xs, ys));
    plot.show();
}

#[test]
fn test_1_x() {
    let mut xs = [0.; 101];

    let mut add = -50f64;
    for idx in 0..=100 {
        xs[idx] = add / 100.;
        add += 1.;
    }

    let mut ys = [0.; 101];
    for (i, y) in ys.iter_mut().enumerate() {
        if xs[i] != 0. {
            *y = 1. / xs[i];
        }
    }

    let plot = Plot::new((xs, ys));
    plot.show();
}

#[test]
fn test_tanh() {
    let plot = Plot::new(|x: f64| x.tanh());
    plot.show()
}
