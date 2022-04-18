#[cfg(target_os="linux")]
use std::thread::JoinHandle;

use macroquad::prelude::Conf;

use crate::{render, LineDesc};

pub type Matrix = Vec<Vec<f64>>;

#[derive(Default)]
pub struct AxisDesc {
    pub title: String,
    pub x_label: String,
    pub y_label: String,
}

pub struct Plot {
    pub xs: Matrix,
    pub ys: Matrix,
    pub line_desc: Vec<LineDesc>,
    pub axis_desc: AxisDesc,
}

impl Plot {
    pub fn new<A: PlotArg>(args: A) -> Plot {
        args.as_plot()
    }

    pub fn add<A: PlotArg>(&mut self, args: A) {
        let plot = args.as_plot();
        self.xs.push(plot.xs[0].clone());
        self.ys.push(plot.ys[0].clone());
        self.line_desc.push(plot.line_desc[0])
    }

    pub fn set_title(&mut self, title: &str) {
        self.axis_desc.title = title.to_string();
    }

    pub fn set_xlabel(&mut self, label: &str) {
        self.axis_desc.x_label = label.to_string();
    }

    pub fn set_ylabel(&mut self, label: &str) {
        self.axis_desc.y_label = label.to_string();
    }

    pub fn show(self) {
        let conf = Conf {
            window_title: self.axis_desc.title.clone(),
            window_width: 395,
            window_height: 395,
            ..Default::default()
        };
        macroquad::Window::from_config(conf, render::run(self));
    }

    #[cfg(target_os="linux")]
    pub fn show_threaded(self) -> JoinHandle<()> {
        std::thread::spawn(|| {
            let conf = Conf {
                window_title: self.axis_desc.title.clone(),
                window_width: 395,
                window_height: 395,
                ..Default::default()
            };
            macroquad::Window::from_config(conf, render::run(self));
        })
    }
}

#[derive(Clone, Copy)]
pub struct XEnd(pub f64);
pub struct YEnd(f64, f64);

/// sets the absolute max value for x
pub fn x(end_x: f64) -> XEnd {
    XEnd(end_x.abs())
}

pub trait PlotArg {
    fn as_plot(&self) -> Plot;
}

impl<const N: usize> PlotArg for ([f64; N], [f64; N]) {
    fn as_plot(&self) -> Plot {
        Plot {
            xs: vec![self.0.to_vec()],
            ys: vec![self.1.to_vec()],
            line_desc: vec![Default::default()],
            axis_desc: Default::default(),
        }
    }
}

impl<const N: usize> PlotArg for ([f64; N], [f64; N], &str) {
    fn as_plot(&self) -> Plot {
        Plot {
            xs: vec![self.0.to_vec()],
            ys: vec![self.1.to_vec()],
            line_desc: vec![self.2.into()],
            axis_desc: Default::default(),
        }
    }
}

impl<const N: usize> PlotArg for [f64; N] {
    fn as_plot(&self) -> Plot {
        Plot {
            xs: vec![(0..N).map(|x| x as f64).collect()],
            ys: vec![self.to_vec()],
            line_desc: vec![Default::default()],
            axis_desc: Default::default(),
        }
    }
}

impl<const N: usize> PlotArg for ([f64; N], &str) {
    fn as_plot(&self) -> Plot {
        Plot {
            xs: vec![(0..N).map(|x| x as f64).collect()],
            ys: vec![self.0.to_vec()],
            line_desc: vec![self.1.into()],
            axis_desc: Default::default(),
        }
    }
}

impl PlotArg for Vec<f64> {
    fn as_plot(&self) -> Plot {
        Plot {
            xs: vec![(0..self.len()).map(|x| x as f64).collect()],
            ys: vec![self.clone()],
            line_desc: vec![Default::default()],
            axis_desc: Default::default(),
        }
    }
}

impl PlotArg for (Vec<f64>, &str) {
    fn as_plot(&self) -> Plot {
        Plot {
            xs: vec![(0..self.0.len()).map(|x| x as f64).collect()],
            ys: vec![self.0.clone()],
            line_desc: vec![self.1.into()],
            axis_desc: Default::default(),
        }
    }
}

impl<F: Fn(f64) -> f64> PlotArg for F {
    fn as_plot(&self) -> Plot {
        let mut xs = [0.; 20000];

        let mut add = -10000f64;
        for x in &mut xs {
            *x = add / 10000.;
            add += 1.;
        }

        let mut ys = [0.; 20000];
        for (i, y) in ys.iter_mut().enumerate() {
            *y = self(xs[i]);
        }
        Plot {
            xs: vec![xs.to_vec()],
            ys: vec![ys.to_vec()],
            line_desc: vec![Default::default()],
            axis_desc: Default::default(),
        }
    }
}

impl<F: Fn(f64) -> f64> PlotArg for (F, XEnd) {
    fn as_plot(&self) -> Plot {
        let mut xs = vec![0.; 200];

        let mut add = -100f64;
        for x in &mut xs {
            *x = (add / 100.) * self.1 .0;
            add += 1.;
        }

        let mut ys = vec![0.; 200];
        for (i, y) in ys.iter_mut().enumerate() {
            *y = self.0(xs[i]);
        }
        Plot {
            xs: vec![xs.to_vec()],
            ys: vec![ys.to_vec()],
            line_desc: vec![Default::default()],
            axis_desc: Default::default(),
        }
    }
}

impl<F: Copy + Fn(f64) -> f64> PlotArg for (F, XEnd, &str) {
    fn as_plot(&self) -> Plot {
        let mut plot = (self.0, self.1).as_plot();
        plot.line_desc = vec![self.2.into()];
        plot
    }
}

impl<F: Copy + Fn(f64) -> f64> PlotArg for (F, &str) {
    fn as_plot(&self) -> Plot {
        let mut plot = self.0.as_plot();
        plot.line_desc = vec![self.1.into()];
        plot
    }
}

impl<F: Fn(f64) -> f64> PlotArg for (F, usize) {
    fn as_plot(&self) -> Plot {
        let mut xs = vec![0.; 20001];

        let mut add = -10000.;
        for x in &mut xs {
            *x = add / self.1 as f64;
            add += 1.;
        }

        let mut ys = vec![0.; 20001];
        for (i, y) in ys.iter_mut().enumerate() {
            *y = self.0(xs[i]);
        }

        Plot {
            xs: vec![xs],
            ys: vec![ys],
            line_desc: vec![Default::default()],
            axis_desc: Default::default(),
        }
    }
}

impl<F: Copy + Fn(f64) -> f64> PlotArg for (F, usize, &str) {
    fn as_plot(&self) -> Plot {
        let mut plot = (self.0, self.1).as_plot();
        plot.line_desc = vec![self.2.into()];
        plot
    }
}
