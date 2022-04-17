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
mod render;

pub use line_desc::*;
pub use plot::*;
pub use render::*;
