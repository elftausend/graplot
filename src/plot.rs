use macroquad::prelude::Conf;

use crate::render;


pub trait PlotArg {
    fn as_plot(&self) -> Plot;
}

impl<const N: usize> PlotArg for ([f64; N], [f64; N]) {
    fn as_plot(&self) -> Plot {
        Plot { xs: self.0.to_vec(), ys: self.1.to_vec(), marker: String::new() }
    }
}

impl<const N: usize> PlotArg for ([f64; N], [f64; N], &str) {
    fn as_plot(&self) -> Plot {
        Plot { xs: self.0.to_vec(), ys: self.1.to_vec(), marker: self.2.to_string() }
    }
}

impl<const N: usize> PlotArg for [f64; N] {
    fn as_plot(&self) -> Plot {
        Plot { xs: (0..N).map(|x| x as f64).collect(), ys: self.to_vec(), marker: String::new() }
    }
}

impl<const N: usize> PlotArg for ([f64; N], &str) {
    fn as_plot(&self) -> Plot {
        Plot { xs: (0..N).map(|x| x as f64).collect(), ys: self.0.to_vec(), marker: self.1.to_string() }
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

    pub fn show(self) {
        let conf = Conf {
            window_width: 325,
            window_height: 325,
            ..Default::default()
        };
        macroquad::Window::from_config(conf, render::run(self.xs, self.ys, self.marker));
        
    }
}