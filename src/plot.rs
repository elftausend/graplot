use std::thread::JoinHandle;

use macroquad::prelude::Conf;

use crate::render;


pub trait PlotArg {
    fn as_plot(&self) -> Plot;
}

impl<const N: usize> PlotArg for ([f64; N], [f64; N]) {
    fn as_plot(&self) -> Plot {
        Plot { xs: self.0.to_vec(), ys: self.1.to_vec(), marker: Default::default() }
    }
}

impl<const N: usize> PlotArg for ([f64; N], [f64; N], &str) {
    fn as_plot(&self) -> Plot {
        Plot { xs: self.0.to_vec(), ys: self.1.to_vec(), marker: self.2.to_string() }
    }
}

impl<const N: usize> PlotArg for [f64; N] {
    fn as_plot(&self) -> Plot {
        Plot { xs: (0..N).map(|x| x as f64).collect(), ys: self.to_vec(), marker: Default::default() }
    }
}

impl PlotArg for Vec<f64> {
    fn as_plot(&self) -> Plot {
        Plot { xs: (0..self.len()).map(|x| x as f64).collect(), ys: self.clone(), marker: Default::default() }
    }
}

impl<const N: usize> PlotArg for ([f64; N], &str) {
    fn as_plot(&self) -> Plot {
        Plot { xs: (0..N).map(|x| x as f64).collect(), ys: self.0.to_vec(), marker: self.1.to_string() }
    }
}

impl <F: Fn(f64) -> f64>PlotArg for F {
    fn as_plot(&self) -> Plot {
        let mut xs = [0.; 20000]; 
    
        let mut add = -10000f64;
        for idx in 0..20000 {
            xs[idx] = add/10000.;
            add += 1.;
        }

        let mut ys = [0.; 20000];
        for (i, y) in ys.iter_mut().enumerate() {
            *y = self(xs[i]);
        }
        Plot { xs: xs.to_vec(), ys: ys.to_vec(), marker: Default::default() }
    }
}

impl <F: Fn(f64) -> f64>PlotArg for (F, usize ) {
    fn as_plot(&self) -> Plot {
        let mut xs = vec![0.; 20001]; 
    
        let mut add = -10000.;
        for idx in 0..=20000 {
            xs[idx] = add / self.1 as f64;
            add += 1.;
        }

        let mut ys = vec![0.; 20001];
        for (i, y) in ys.iter_mut().enumerate() {
            *y = self.0(xs[i]);
        }

        Plot { xs, ys, marker: Default::default() }
    }
}

pub struct Plot {
    xs: Vec<f64>,
    ys: Vec<f64>,
    marker: String,
}

impl Plot {
    pub fn new<A: PlotArg>(arguments: A) -> Plot {
        arguments.as_plot()
    }

    pub fn show(self) -> JoinHandle<()> {
        std::thread::spawn(|| {
            let conf = Conf {
                window_width: 325,
                window_height: 325,
                ..Default::default()
            };
            macroquad::Window::from_config(conf, render::run(self.xs, self.ys, self.marker));
        })
        
    }
}