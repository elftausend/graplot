use std::thread::JoinHandle;

use macroquad::prelude::Conf;

use crate::{render, LineDesc};


pub trait PlotArg {
    fn as_plot(&self) -> Plot;
}

impl<const N: usize> PlotArg for ([f64; N], [f64; N]) {
    fn as_plot(&self) -> Plot {
        Plot { xs: self.0.to_vec(), ys: self.1.to_vec(), line_desc: Default::default() }
    }
}

impl<const N: usize> PlotArg for ([f64; N], [f64; N], &str) {
    fn as_plot(&self) -> Plot {
        Plot { xs: self.0.to_vec(), ys: self.1.to_vec(), line_desc: self.2.into() }
    }
}

impl<const N: usize> PlotArg for [f64; N] {
    fn as_plot(&self) -> Plot {
        Plot { xs: (0..N).map(|x| x as f64).collect(), ys: self.to_vec(), line_desc: Default::default() }
    }
}

impl PlotArg for Vec<f64> {
    fn as_plot(&self) -> Plot {
        Plot { xs: (0..self.len()).map(|x| x as f64).collect(), ys: self.clone(), line_desc: Default::default() }
    }
}

impl PlotArg for (Vec<f64>, &str) {
    fn as_plot(&self) -> Plot {
        Plot { xs: (0..self.0.len()).map(|x| x as f64).collect(), ys: self.0.clone(), line_desc: self.1.into() }
    }
}

impl<const N: usize> PlotArg for ([f64; N], &str) {
    fn as_plot(&self) -> Plot {
        Plot { xs: (0..N).map(|x| x as f64).collect(), ys: self.0.to_vec(), line_desc: self.1.into() }
    }
}

impl<F: Fn(f64) -> f64> PlotArg for F {
    fn as_plot(&self) -> Plot {
        let mut xs = [0.; 20000]; 
    
        let mut add = -10000f64;
        for value in &mut xs {
            *value = add/10000.;
            add += 1.;
        }

        let mut ys = [0.; 20000];
        for (i, y) in ys.iter_mut().enumerate() {
            *y = self(xs[i]);
        }
        Plot { xs: xs.to_vec(), ys: ys.to_vec(), line_desc: Default::default() }
    }
}

impl<F: Fn(f64) -> f64> PlotArg for (F, usize ) {
    fn as_plot(&self) -> Plot {
        let mut xs = vec![0.; 20001]; 
    
        let mut add = -10000.;
        for value in &mut xs {
            *value = add / self.1 as f64;
            add += 1.;
        }

        let mut ys = vec![0.; 20001];
        for (i, y) in ys.iter_mut().enumerate() {
            *y = self.0(xs[i]);
        }

        Plot { xs, ys, line_desc: Default::default() }
    }
}

pub struct Plot {
    pub xs: Vec<f64>,
    pub ys: Vec<f64>,
    pub line_desc: LineDesc,
}

impl Plot {
    pub fn new<A: PlotArg>(arguments: A) -> Plot {
        arguments.as_plot()
    }

    pub fn show(self) {        
        let conf = Conf {
            window_width: 365,
            window_height: 365,
            ..Default::default()
        };
        macroquad::Window::from_config(conf, render::run(self));
    }

    pub fn show_threaded(self) -> JoinHandle<()> {
        std::thread::spawn(|| {
            let conf = Conf {
                window_width: 365,
                window_height: 365,
                ..Default::default()
            };
            macroquad::Window::from_config(conf, render::run(self));
        })
    }
}