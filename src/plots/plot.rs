//#[cfg(target_os = "linux")]
//use std::thread::JoinHandle;

use litequad::prelude::{Color, Conf};

use crate::{render, LineDesc, Matrix, AxisDesc, Desc, XEnd, ToF64};

#[derive(Clone, Default)]
pub struct Plot {
    pub xs: Matrix,
    pub ys: Matrix,
    pub line_desc: Vec<LineDesc>,
    pub axis_desc: AxisDesc,
    pub desc: Desc,
}

impl Plot {
    pub fn new(args: impl PlotArg) -> Plot {
        args.as_plot()
    }

    pub fn set_desc(&mut self, desc: Desc) {
        self.desc = desc;
    }

    /// Set graph color
    /// # Example
    /// ```
    /// use graplot::Plot;
    ///
    /// let mut plot = Plot::new([1., 2., 3.]);
    /// plot.set_color(0., 0.78, 1.);
    /// plot.show();
    /// ```
    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.line_desc[0].color = Color::new(r, g, b, 1.);
    }

    /// Colors the graph at the given index with the color.
    /// # Example
    /// ```
    /// use graplot::{Plot, BLUE};
    /// 
    /// let mut plot = Plot::new([1., 2., 3.,]);
    /// plot.add([2., 3., 4.,]);
    /// plot.color(1, BLUE);
    /// plot.show();
    /// ```
    pub fn color(&mut self, idx: usize, color: Color) {
        self.line_desc[idx].color = color;
    }

    /// adds a new graph / plot to the original plot.
    pub fn add<A: PlotArg>(&mut self, args: A) {
        let plot = args.as_plot();
        self.xs.push(plot.xs[0].clone());
        self.ys.push(plot.ys[0].clone());
        self.line_desc.push(plot.line_desc[0])
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
        
        /*let mut max_y = max_matrix(&self.ys);
        max_y = max_display(max_y, false);

        let steps_y = get_steps(max_y, self.desc.min_steps_x.into());
        let window_height = (steps_y * self.desc.spacing_y as f64).max(395.) as i32;

        let mut max_x = max_matrix(&self.xs);
        max_x = max_display(max_x, false);

        let steps_x = get_steps(max_x, self.desc.min_steps_x.into());
        let window_width = (steps_x * self.desc.spacing_x as f64).max(395.) as i32 * 2 + 7;
        */
        let conf = Conf {
            window_title: self.axis_desc.title.clone(),
            window_width: 395,
            window_height: 395,
            ..Default::default()
        };
        litequad::Window::from_config(conf, render::plot::run(self, 0., false));
    }

    /* 
    #[cfg(target_os = "linux")]
    pub fn show_threaded(self) -> JoinHandle<()> {
        std::thread::spawn(|| {
            let conf = Conf {
                window_title: self.axis_desc.title.clone(),
                window_width: 395,
                window_height: 395,
                ..Default::default()
            };
            litequad::Window::from_config(conf, render::plot::run(self, 0.));
        })
    }
    */
}


pub trait PlotArg {
    fn as_plot(&self) -> Plot;
}

impl PlotArg for () {
    fn as_plot(&self) -> Plot {
        Default::default()
    }
}

impl<T: ToF64<Output = f64>, const N: usize> PlotArg for ([T; N], [T; N]) {
    fn as_plot(&self) -> Plot {
        Plot {
            xs: vec![self.0.to_f64()],
            ys: vec![self.1.to_f64()],
            line_desc: vec![Default::default()],
            axis_desc: Default::default(),
            desc: Default::default(),
        }
    }
}

impl<T: ToF64<Output = f64>, const N: usize> PlotArg for ([T; N], [T; N], &str) {
    fn as_plot(&self) -> Plot {
        Plot {
            xs: vec![self.0.to_f64()],
            ys: vec![self.1.to_f64()],
            line_desc: vec![self.2.into()],
            axis_desc: Default::default(),
            desc: Default::default(),
        }
    }
}

impl<T: ToF64<Output = f64>, const N: usize> PlotArg for [T; N] {
    fn as_plot(&self) -> Plot {
        Plot {
            xs: vec![(0..N).map(|x| x as f64).collect()],
            ys: vec![self.to_f64()],
            line_desc: vec![Default::default()],
            axis_desc: Default::default(),
            desc: Default::default(),
        }
    }
}

impl<T: ToF64<Output = f64>, const N: usize> PlotArg for ([T; N], &str) {
    fn as_plot(&self) -> Plot {
        Plot {
            xs: vec![(0..N).map(|x| x as f64).collect()],
            ys: vec![self.0.to_f64()],
            line_desc: vec![self.1.into()],
            axis_desc: Default::default(),
            desc: Default::default(),
        }
    }
}

impl<T: ToF64<Output = f64>, const N: usize> PlotArg for (&[T; N], &[T; N]) {
    fn as_plot(&self) -> Plot {
        Plot {
            xs: vec![self.0.to_f64()],
            ys: vec![self.1.to_f64()],
            line_desc: vec![Default::default()],
            axis_desc: Default::default(),
            desc: Default::default(),
        }
    }
}

impl<T: ToF64<Output = f64>, const N: usize> PlotArg for (&[T; N], &[T; N], &str) {
    fn as_plot(&self) -> Plot {
        Plot {
            xs: vec![self.0.to_f64()],
            ys: vec![self.1.to_f64()],
            line_desc: vec![self.2.into()],
            axis_desc: Default::default(),
            desc: Default::default(),
        }
    }
}

impl<T: ToF64<Output = f64>, const N: usize> PlotArg for &[T; N] {
    fn as_plot(&self) -> Plot {
        Plot {
            xs: vec![(0..N).map(|x| x as f64).collect()],
            ys: vec![self.to_f64()],
            line_desc: vec![Default::default()],
            axis_desc: Default::default(),
            desc: Default::default(),
        }
    }
}

impl<T: ToF64<Output = f64>, const N: usize> PlotArg for (&[T; N], &str) {
    fn as_plot(&self) -> Plot {
        Plot {
            xs: vec![(0..N).map(|x| x as f64).collect()],
            ys: vec![self.0.to_f64()],
            line_desc: vec![self.1.into()],
            axis_desc: Default::default(),
            desc: Default::default(),
        }
    }
}

impl<T: ToF64<Output = f64>> PlotArg for (&[T], &[T]) {
    fn as_plot(&self) -> Plot {
        Plot {
            xs: vec![self.0.to_f64()],
            ys: vec![self.1.to_f64()],
            line_desc: vec![Default::default()],
            axis_desc: Default::default(),
            desc: Default::default(),
        }
    }
}

impl<T: ToF64<Output = f64>> PlotArg for (&[T], &[T], &str) {
    fn as_plot(&self) -> Plot {
        Plot {
            xs: vec![self.0.to_f64()],
            ys: vec![self.1.to_f64()],
            line_desc: vec![self.2.into()],
            axis_desc: Default::default(),
            desc: Default::default(),
        }
    }
}

impl<T: ToF64<Output = f64>> PlotArg for &[T] {
    fn as_plot(&self) -> Plot {
        Plot {
            xs: vec![(0..self.len()).map(|x| x as f64).collect()],
            ys: vec![self.to_f64()],
            line_desc: vec![Default::default()],
            axis_desc: Default::default(),
            desc: Default::default(),
        }
    }
}

impl<T: ToF64<Output = f64>> PlotArg for (&[T], &str) {
    fn as_plot(&self) -> Plot {
        Plot {
            xs: vec![(0..self.0.len()).map(|x| x as f64).collect()],
            ys: vec![self.0.to_f64()],
            line_desc: vec![self.1.into()],
            axis_desc: Default::default(),
            desc: Default::default(),
        }
    }
}

impl<T: ToF64<Output = f64>> PlotArg for (Vec<T>, &str) {
    fn as_plot(&self) -> Plot {
        Plot {
            xs: vec![(0..self.0.len()).map(|x| x as f64).collect()],
            ys: vec![self.0.to_f64()],
            line_desc: vec![self.1.into()],
            axis_desc: Default::default(),
            desc: Default::default(),
        }
    }
}

impl<T: ToF64<Output = f64>> PlotArg for (&Vec<T>, &Vec<T>) {
    fn as_plot(&self) -> Plot {
        Plot {
            xs: vec![self.0.to_f64()],
            ys: vec![self.1.to_f64()],
            line_desc: vec![Default::default()],
            axis_desc: Default::default(),
            desc: Default::default(),
        }
    }
}

impl<T: ToF64<Output = f64>> PlotArg for Vec<T> {
    fn as_plot(&self) -> Plot {
        Plot {
            xs: vec![(0..self.len()).map(|x| x.to_f64()).collect()],
            ys: vec![self.to_f64()],
            line_desc: vec![Default::default()],
            axis_desc: Default::default(),
            desc: Default::default(),
        }
    }
}

impl<T: ToF64<Output = f64>> PlotArg for (Vec<T>, Vec<T>) {
    fn as_plot(&self) -> Plot {
        Plot {
            xs: vec![self.0.to_f64()],
            ys: vec![self.1.to_f64()],
            line_desc: vec![Default::default()],
            axis_desc: Default::default(),
            desc: Default::default(),
        }
    }
}

impl<T: ToF64<Output = f64>> PlotArg for (Vec<T>, Vec<T>, &str) {
    fn as_plot(&self) -> Plot {
        Plot {
            xs: vec![self.0.to_f64()],
            ys: vec![self.1.to_f64()],
            line_desc: vec![self.2.into()],
            axis_desc: Default::default(),
            desc: Default::default(),
        }
    }
}

impl<F: Fn(f64) -> f64> PlotArg for F {
    fn as_plot(&self) -> Plot {
        let mut xs = [0.; 20000];

        let mut add = -10000f64;
        for x in &mut xs {
            *x = add / 10000.;
            add += 1.;
        }

        let mut ys = [0.; 20000];
        for (i, y) in ys.iter_mut().enumerate() {
            *y = self(xs[i]);
        }
        Plot {
            xs: vec![xs.to_vec()],
            ys: vec![ys.to_vec()],
            line_desc: vec![Default::default()],
            axis_desc: Default::default(),
            desc: Default::default(),
        }
    }
}

impl<F: Fn(f64) -> f64> PlotArg for (F, Option<XEnd>) {
    fn as_plot(&self) -> Plot {
        let mut xs = vec![0.; 200];

        let mut add = -100f64;
        for x in &mut xs {
            *x = (add / 100.) * self.1.unwrap().0;
            add += 1.;
        }

        let mut ys = vec![0.; 200];
        for (i, y) in ys.iter_mut().enumerate() {
            *y = self.0(xs[i]);
        }
        Plot {
            xs: vec![xs.to_vec()],
            ys: vec![ys.to_vec()],
            line_desc: vec![Default::default()],
            axis_desc: Default::default(),
            desc: Default::default(),
        }
    }
}

impl<F: Copy + Fn(f64) -> f64> PlotArg for (F, Option<XEnd>, &str) {
    fn as_plot(&self) -> Plot {
        let mut plot = (self.0, self.1).as_plot();
        plot.line_desc = vec![self.2.into()];
        plot
    }
}

impl<F: Copy + Fn(f64) -> f64> PlotArg for (F, &str, Option<XEnd>) {
    fn as_plot(&self) -> Plot {
        let mut plot = (self.0, self.2).as_plot();
        plot.line_desc = vec![self.1.into()];
        plot
    }
}

impl<F: Copy + Fn(f64) -> f64> PlotArg for (F, &str) {
    fn as_plot(&self) -> Plot {
        let mut plot = self.0.as_plot();
        plot.line_desc = vec![self.1.into()];
        plot
    }
}

impl<F: Fn(f64) -> f64> PlotArg for (F, usize) {
    fn as_plot(&self) -> Plot {
        let mut xs = vec![0.; 20001];

        let mut add = -10000.;
        for x in &mut xs {
            *x = add / self.1 as f64;
            add += 1.;
        }

        let mut ys = vec![0.; 20001];
        for (i, y) in ys.iter_mut().enumerate() {
            *y = self.0(xs[i]);
        }

        Plot {
            xs: vec![xs],
            ys: vec![ys],
            line_desc: vec![Default::default()],
            axis_desc: Default::default(),
            desc: Default::default(),
        }
    }
}

impl PlotArg for Plot {
    fn as_plot(&self) -> Plot {
        self.clone()
    }
}

impl<F: Copy + Fn(f64) -> f64> PlotArg for (F, usize, &str) {
    fn as_plot(&self) -> Plot {
        let mut plot = (self.0, self.1).as_plot();
        plot.line_desc = vec![self.2.into()];
        plot
    }
}
