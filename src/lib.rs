mod plot;
mod render;

use macroquad::color::{Color, GREEN, RED, BLUE, YELLOW};
pub use plot::*;
pub use render::*;

#[derive(PartialEq,)]
pub enum Marker {
    Circle(f32),
    None
}

pub enum LineType {
    Solid,
    None,
}

pub struct LineDesc {
    pub color: Color,
    pub marker: Marker,
    pub line_type: LineType
}

impl Default for LineDesc {
    fn default() -> Self {
        LineDesc { color: GREEN, marker: Marker::None, line_type: LineType::Solid }
    }
}

impl From<&str> for LineDesc {
    fn from(desc: &str) -> LineDesc {
        let mut color = GREEN;
        let mut line_type = LineType::None;
        let mut marker = Marker::None;

        for char in desc.chars() {
            match char {
                'r' => color = RED,
                'g' => color = GREEN,
                'b' => color = BLUE,
                'y' => color = YELLOW,
                //cyan
                'c' => color = Color::new(0., 1., 1., 1.), 
                'o' => marker = Marker::Circle(5.),
                '-' => line_type = LineType::Solid,
               _ => {}
            }
        }
        if marker == Marker::None {
            line_type = LineType::Solid
        }
        LineDesc { color, marker, line_type }
    }
}

