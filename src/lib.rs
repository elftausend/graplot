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
pub mod graph;

pub use graph::Graph;
pub use plots::*;
pub use line_desc::*;
pub use polynomial::*;
pub use render::*;
pub use eval::*;
pub use litequad::color::colors::*;
pub use litequad::color::Color;

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
pub fn x(end_x: f64) -> Option<XEnd> {
    Some(XEnd(end_x.abs()))
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Desc {
    pub end_x: Option<XEnd>,
    pub spacing_x: f32, // pixels
    pub spacing_y: f32, // pixels
    pub min_steps_x: f32,
    pub min_steps_y: f32,
}

impl Default for Desc {
    fn default() -> Self {
        Self {
            end_x: None,
            spacing_x: 40.,
            spacing_y: 40.,
            min_steps_x: 4.,
            min_steps_y: 4.,
        }
    }
}

pub(crate) trait ToF64 {
    type Output;
    fn to_f64(&self) -> Self::Output;
}

impl<T: ToF64<Output = f64>> ToF64 for [T] {
    type Output = Vec<f64>;

    fn to_f64(&self) -> Self::Output {
        self.iter().map(|val| val.to_f64()).collect()
    }
}

macro_rules! impl_tof64 {
    ($($t:ty),*) => {
        $(
            impl ToF64 for $t {
                type Output = f64;
                #[inline]
                fn to_f64(&self) -> f64 {
                    *self as f64
                }
            }
        )*
    };
}

impl_tof64!(f32, f64, i8, i16, i32, i64, i128,
    isize, u8, u16, u32, u64, u128, usize
);