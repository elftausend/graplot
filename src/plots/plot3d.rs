use litequad::prelude::Conf;

use crate::{render, Matrix, LineDesc, Desc};


pub struct Plot3D {
    pub xs: Matrix,
    pub ys: Matrix,
    pub zs: Matrix,
    pub line_desc: Vec<LineDesc>,
    pub desc: Desc,
}

impl Plot3D {
    pub fn new<A: Plot3DArg>(args: A) -> Plot3D {
        args.as_plot()
    }

    pub fn add<A: Plot3DArg>(&mut self, args: A) {
        let plot = args.as_plot();
        self.xs.push(plot.xs[0].clone());
        self.ys.push(plot.ys[0].clone());
        self.zs.push(plot.zs[0].clone());
        self.line_desc.push(plot.line_desc[0])
    }

    pub fn show(self) {
        let conf = Conf {
            window_width: 595,
            window_height: 595,
            ..Default::default()
        };
        litequad::Window::from_config(conf, render::plot3d::run(self));
    }
}

pub trait Plot3DArg {
    fn as_plot(&self) -> Plot3D;
}

impl<const N: usize> Plot3DArg for ([f64; N], [f64; N], [f64; N]) {
    fn as_plot(&self) -> Plot3D {
        Plot3D {
            xs: vec![self.0.to_vec()],
            ys: vec![self.1.to_vec()],
            zs: vec![self.2.to_vec()],
            line_desc: vec![Default::default()],
            desc: Default::default(),
        }
    }
}

impl<const N: usize> Plot3DArg for ([f64; N], [f64; N], [f64; N], &str) {
    fn as_plot(&self) -> Plot3D {
        Plot3D {
            xs: vec![self.0.to_vec()],
            ys: vec![self.1.to_vec()],
            zs: vec![self.2.to_vec()],
            line_desc: vec![self.3.into()],
            desc: Default::default(),
        }
    }
}

impl<const N: usize> Plot3DArg for (&[f64; N], &[f64; N], &[f64; N], &str) {
    fn as_plot(&self) -> Plot3D {
        Plot3D {
            xs: vec![self.0.to_vec()],
            ys: vec![self.1.to_vec()],
            zs: vec![self.2.to_vec()],
            line_desc: vec![self.3.into()],
            desc: Default::default(),
        }
    }
}

impl<const N: usize> Plot3DArg for (&[f64; N], &[f64; N], &[f64; N]) {
    fn as_plot(&self) -> Plot3D {
        Plot3D {
            xs: vec![self.0.to_vec()],
            ys: vec![self.1.to_vec()],
            zs: vec![self.2.to_vec()],
            line_desc: vec![Default::default()],
            desc: Default::default(),
        }
    }
}

impl Plot3DArg for (&[f64], &[f64], &[f64], &str) {
    fn as_plot(&self) -> Plot3D {
        Plot3D {
            xs: vec![self.0.to_vec()],
            ys: vec![self.1.to_vec()],
            zs: vec![self.2.to_vec()],
            line_desc: vec![self.3.into()],
            desc: Default::default(),
        }
    }
}

impl Plot3DArg for (&[f64], &[f64], &[f64]) {
    fn as_plot(&self) -> Plot3D {
        Plot3D {
            xs: vec![self.0.to_vec()],
            ys: vec![self.1.to_vec()],
            zs: vec![self.2.to_vec()],
            line_desc: vec![Default::default()],
            desc: Default::default(),
        }
    }
}