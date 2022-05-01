use litequad::prelude::{Color, Conf};

use crate::render;


pub struct Pie {
    pub segs: Vec<PieSegment>,
    pub title: String,
    pub radius: f64
}

impl Pie {
    pub fn new<A: PieSegs>(args: A) -> Pie {
        Pie {
            segs: args.as_pie_segs(),
            radius: 110.,
            title: Default::default()
        }
    }

    /// sets the radius of the final circle.
    /// Default radius: 110.0
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    pub fn show(self) {
        let conf = Conf {
            window_title: self.title.clone(),
            window_width: 395,
            window_height: 395,
            ..Default::default()
        };
        litequad::Window::from_config(conf, render::pie::run(self));
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct PieSegment {
    pub percentage: f64,
    pub label: String,
    pub color: Option<Color>
}

impl PartialOrd for PieSegment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.percentage.partial_cmp(&other.percentage) 
    }
}

pub trait PieSegs {
    fn as_pie_segs(&self) -> Vec<PieSegment>;
}

impl PieSegs for Pie {
    fn as_pie_segs(&self) -> Vec<PieSegment> {
        self.segs.clone()
    }
}

impl PieSegs for Vec<PieSegment> {
    fn as_pie_segs(&self) -> Vec<PieSegment> {
        self.clone()
    }
}

impl PieSegs for &[PieSegment] {
    fn as_pie_segs(&self) -> Vec<PieSegment> {
        self.to_vec()
    }
}

impl <const N: usize>PieSegs for &[PieSegment; N] {
    fn as_pie_segs(&self) -> Vec<PieSegment> {
        self.to_vec()
    }
}

impl <const N: usize>PieSegs for [PieSegment; N] {
    fn as_pie_segs(&self) -> Vec<PieSegment> {
        self.to_vec()
    }
}

impl PieSegs for &[f64] {
    fn as_pie_segs(&self) -> Vec<PieSegment> {
        self.iter()
            .map(|per| PieSegment {
                percentage: *per,
                label: Default::default(),
                color: None
            }
        ).collect()
    }
}

impl<const N: usize> PieSegs for &[f64; N] {
    fn as_pie_segs(&self) -> Vec<PieSegment> {
        self.iter()
            .map(|per| PieSegment {
                percentage: *per,
                label: Default::default(),
                color: None
            }
        ).collect()
    }
}

impl<const N: usize> PieSegs for [f64; N] {
    fn as_pie_segs(&self) -> Vec<PieSegment> {
        self.iter()
            .map(|per| PieSegment {
                percentage: *per,
                label: Default::default(),
                color: None
            }
        ).collect()
    }
}

impl PieSegs for Vec<f64> {
    fn as_pie_segs(&self) -> Vec<PieSegment> {
        self.iter()
            .map(|per| PieSegment {
                percentage: *per,
                label: Default::default(),
                color: None
            }
        ).collect()
    }
}

impl PieSegs for (&[f64], &str) {
    fn as_pie_segs(&self) -> Vec<PieSegment> {
        self.0.iter()
            .map(|per| PieSegment {
                percentage: *per,
                label: self.1.to_string(),
                color: None
            }
        ).collect()
    }
}

impl<const N: usize> PieSegs for (&[f64; N], &str) {
    fn as_pie_segs(&self) -> Vec<PieSegment> {
        self.0.iter()
            .map(|per| PieSegment {
                percentage: *per,
                label: self.1.to_string(),
                color: None
            }
        ).collect()
    }
}

impl<const N: usize> PieSegs for ([f64; N], &str) {
    fn as_pie_segs(&self) -> Vec<PieSegment> {
        self.0.iter()
            .map(|per| PieSegment {
                percentage: *per,
                label: self.1.to_string(),
                color: None
            }
        ).collect()
    }
}

impl PieSegs for (Vec<f64>, &str) {
    fn as_pie_segs(&self) -> Vec<PieSegment> {
        self.0.iter()
            .map(|per| PieSegment {
                percentage: *per,
                label: self.1.to_string(),
                color: None
            }
        ).collect()
    }
}