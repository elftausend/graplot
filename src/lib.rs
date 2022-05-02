//!
//! 'graplot' is an experimental plotting library written in Rust that is based on [macroquad].
//! It creates a window displaying the graphs.
//!
//! # Example
//! ```
//! use graplot::Plot;
//!
//!
//! let plot = Plot::new(|x: f64| x.powf(2.));
//! plot.show();
//!
//! ```
//!

mod line_desc;
mod plots;
mod polynomial;
mod render;
mod eval;

pub use plots::*;
pub use line_desc::*;
pub use polynomial::*;
pub use render::*;
pub use eval::*;
pub use litequad::color::colors::*;

pub type Matrix = Vec<Vec<f64>>;

#[derive(Default, Clone)]
pub struct AxisDesc {
    pub title: String,
    pub x_label: String,
    pub y_label: String,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct XEnd(pub f64);
pub struct YEnd(f64, f64);

/// sets the absolute max value for x
pub fn x(end_x: f64) -> XEnd {
    XEnd(end_x.abs())
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Desc {
    pub end: XEnd,
    pub spacing_x: f32, // pixels
    pub spacing_y: f32, // pixels
    pub min_steps_x: f32,
    pub min_steps_y: f32,
}

impl Default for Desc {
    fn default() -> Self {
        Self {
            end: x(1.),
            spacing_x: 40.,
            spacing_y: 40.,
            min_steps_x: 4.,
            min_steps_y: 4.,
        }
    }
}