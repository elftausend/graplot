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
mod plot;
mod polynomial;
mod render;
mod scatter;
mod eval;
mod bar;

pub use line_desc::*;
pub use plot::*;
pub use polynomial::*;
pub use render::*;
pub use scatter::*;
pub use eval::*;
pub use bar::*;

pub type Matrix = Vec<Vec<f64>>;

#[derive(Default, Clone)]
pub struct AxisDesc {
    pub title: String,
    pub x_label: String,
    pub y_label: String,
}