use litequad::prelude::{Color, Conf};

use crate::render;

/// ```
/// use graplot::Pie;

/// // without labels: let pie = Pie::new([35., 25., 25., 15.]);
/// let draw = [(35., "label"), (25., "len"), (25., "labeled"), (15., "test")];
/// let mut pie = Pie::new(draw);
/// pie.set_title("title");
/// pie.show();
/// ```
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

    /// Colors the segment at the given index with the color.
    /// /// # Example
    /// ```
    /// use graplot::{Pie, Color};
    /// 
    /// let mut pie = Pie::new([25., 20., 15., 15., 35.]);
    /// pie.color(1, Color::new(0.2, 0.1, 0.3, 1.));
    /// pie.show();
    /// ```
    pub fn color(&mut self, idx: usize, color: Color) {
        self.segs[idx].color = Some(color);
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

impl PieSegs for &[(f64, &str)] {
    fn as_pie_segs(&self) -> Vec<PieSegment> {
        self.iter()
            .map(|per| PieSegment {
                percentage: per.0,
                label: per.1.to_string(),
                color: None
            }
        ).collect()
    }
}

impl<const N: usize> PieSegs for &[(f64, &str); N] {
    fn as_pie_segs(&self) -> Vec<PieSegment> {
        self.iter()
            .map(|per| PieSegment {
                percentage: per.0,
                label: per.1.to_string(),
                color: None
            }
        ).collect()
    }
}

impl<const N: usize> PieSegs for [(f64, &str); N] {
    fn as_pie_segs(&self) -> Vec<PieSegment> {
        self.iter()
            .map(|per| PieSegment {
                percentage: per.0,
                label: per.1.to_string(),
                color: None
            }
        ).collect()
    }
}

impl PieSegs for Vec<(f64, &str)> {
    fn as_pie_segs(&self) -> Vec<PieSegment> {
        self.iter()
            .map(|per| PieSegment {
                percentage: per.0,
                label: per.1.to_string(),
                color: None
            }
        ).collect()
    }
}