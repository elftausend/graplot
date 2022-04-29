use litequad::prelude::{Color, Conf};

use crate::{AxisDesc, max_display, get_steps, max, Desc, render};

#[derive(Debug, Clone, PartialEq)]
pub struct BarDesc {
    pub width: f32,
    pub label: String,
    pub color: Color,
}

impl Default for BarDesc {
    fn default() -> Self {
        Self { 
            width: 100., 
            label: Default::default(), 
            color: litequad::color::GREEN }
    }
}

pub trait BarDescArg {
    fn as_bar_desc(&self) -> Vec<BarDesc>;
}

impl BarDescArg for BarDesc {
    fn as_bar_desc(&self) -> Vec<BarDesc> {
        vec![self.clone()]
    }
}

impl BarDescArg for &[BarDesc] {
    fn as_bar_desc(&self) -> Vec<BarDesc> {
        self.to_vec()
    }
}

impl BarDescArg for &str {
    fn as_bar_desc(&self) -> Vec<BarDesc> {
        vec![BarDesc {
            label: self.to_string(),
            ..Default::default()
        }]
    }
}

impl BarDescArg for &[&str] {
    fn as_bar_desc(&self) -> Vec<BarDesc> {
        self.iter()
            .map(|s| s.as_bar_desc()[0].clone())
            .collect()
    }
}

impl<const N: usize> BarDescArg for &[&str; N] {
    fn as_bar_desc(&self) -> Vec<BarDesc> {
        self.iter()
            .map(|s| s.as_bar_desc()[0].clone())
            .collect()
    }
}

impl<const N: usize> BarDescArg for [&str; N] {
    fn as_bar_desc(&self) -> Vec<BarDesc> {
        self.iter()
            .map(|s| s.as_bar_desc()[0].clone())
            .collect()
    }
}

pub struct Bar {
    pub bars: Vec<BarDesc>,
    pub ys: Vec<f64>,
    pub axis_desc: AxisDesc,
    pub desc: Desc,
}

impl Bar {
    pub fn new<A: BarDescArg>(xs: A, ys: &[f64]) -> Bar {
        Bar {
            bars: xs.as_bar_desc(),
            ys: ys.to_vec(),
            axis_desc: AxisDesc::default(),
            desc: Default::default()
        }
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
        let mut max_x = max(&self.ys);
        max_x = max_display(max_x);

        let steps = get_steps(max_x, self.desc.min_steps_x.into());
        
        let window_height = (steps * self.desc.spacing_y as f64).max(395.) as i32;
        let mut window_width = 0.;

        for bar in &self.bars {
            window_width += bar.width;
 
        }

        let conf = Conf {
            window_title: self.axis_desc.title.clone(),
            window_width: window_width.max(395.) as i32 + 100,
            window_height,
            ..Default::default()
        };
        litequad::Window::from_config(conf, render::bar::run(self));
    }
}
