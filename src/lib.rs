//!
//! 'graplot' is a simple and experimental plotting library written in Rust that is based on macroquad.
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

mod plot;
mod render;
mod line_desc;

pub use plot::*;
pub use render::*;
pub use line_desc::*;

