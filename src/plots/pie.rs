
pub struct Pie {
    pub segs: Vec<PieSegment>,
    pub title: String,
}

impl Pie {
    pub fn new<A: PieSegs>(args: A) -> Pie {
        Pie {
            segs: args.as_pie_segs(),
            title: Default::default()
        }
    }
    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct PieSegment {
    percentage: f64,
    label: String,
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
                label: Default::default()
            }
        ).collect()
    }
}

impl<const N: usize> PieSegs for &[f64; N] {
    fn as_pie_segs(&self) -> Vec<PieSegment> {
        self.iter()
            .map(|per| PieSegment {
                percentage: *per,
                label: Default::default()
            }
        ).collect()
    }
}

impl<const N: usize> PieSegs for [f64; N] {
    fn as_pie_segs(&self) -> Vec<PieSegment> {
        self.iter()
            .map(|per| PieSegment {
                percentage: *per,
                label: Default::default()
            }
        ).collect()
    }
}

impl PieSegs for Vec<f64> {
    fn as_pie_segs(&self) -> Vec<PieSegment> {
        self.iter()
            .map(|per| PieSegment {
                percentage: *per,
                label: Default::default()
            }
        ).collect()
    }
}

impl PieSegs for (&[f64], &str) {
    fn as_pie_segs(&self) -> Vec<PieSegment> {
        self.0.iter()
            .map(|per| PieSegment {
                percentage: *per,
                label: self.1.to_string()
            }
        ).collect()
    }
}

impl<const N: usize> PieSegs for (&[f64; N], &str) {
    fn as_pie_segs(&self) -> Vec<PieSegment> {
        self.0.iter()
            .map(|per| PieSegment {
                percentage: *per,
                label: self.1.to_string()
            }
        ).collect()
    }
}

impl<const N: usize> PieSegs for ([f64; N], &str) {
    fn as_pie_segs(&self) -> Vec<PieSegment> {
        self.0.iter()
            .map(|per| PieSegment {
                percentage: *per,
                label: self.1.to_string()
            }
        ).collect()
    }
}

impl PieSegs for (Vec<f64>, &str) {
    fn as_pie_segs(&self) -> Vec<PieSegment> {
        self.0.iter()
            .map(|per| PieSegment {
                percentage: *per,
                label: self.1.to_string()
            }
        ).collect()
    }
}