use litequad::prelude::Conf;

use crate::render;


pub struct Plot3D {

}

impl Plot3D {

    pub fn show(self) {
        let conf = Conf {
            window_width: 595,
            window_height: 595,
            ..Default::default()
        };
        litequad::Window::from_config(conf, render::plot3d::run());
    }
}