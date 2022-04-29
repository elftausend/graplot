use litequad::prelude::Color;

use crate::AxisDesc;

#[derive(Debug, Clone, PartialEq)]
pub struct BarDesc {
    width: f32,
    label: String,
    color: Color,
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

pub struct Bar {
    bars: Vec<BarDesc>,
    ys: Vec<f64>,
    axis_desc: AxisDesc,
}

impl Bar {
    pub fn new<A: BarDescArg>(xs: A, ys: &[f64]) -> Bar {
        Bar {
            bars: xs.as_bar_desc(),
            ys: ys.to_vec(),
            axis_desc: AxisDesc::default()
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
}

