use litequad::prelude::Conf;

use crate::{Plot, PlotArg, LineType, Marker, render, min_matrix, Desc};

pub struct Scatter {
    pub plot: Plot
}

impl Scatter {
    pub fn new<A: PlotArg>(args: A) -> Scatter {
        let mut plot = args.as_plot();
        for line_desc in &mut plot.line_desc {
            line_desc.line_type = LineType::None;
            line_desc.marker = Marker::Circle(4.);
        }
        Scatter {
            plot
        }
    }

    pub fn set_desc(&mut self, desc: Desc) {
        self.plot.set_desc(desc)
    }

    pub fn add<A: PlotArg>(&mut self, args: A) {
        let plot = args.as_plot();
        self.plot.xs.push(plot.xs[0].clone());
        self.plot.ys.push(plot.ys[0].clone());
        self.plot.line_desc.push(plot.line_desc[0])
    }

    pub fn show(self) {
        let conf = Conf {
            window_title: self.plot.axis_desc.title.clone(),
            window_width: 395,
            window_height: 395,
            ..Default::default()
        };
        let min_y = min_matrix(&self.plot.ys);
        litequad::Window::from_config(conf, render::plot::run(self.plot, min_y, true));
    }
}

