use litequad::color::{Color, BLUE, GREEN, RED, YELLOW};

#[derive(PartialEq, Clone, Copy)]
pub enum Marker {
    Circle(f32),
    None,
}

#[derive(Clone, Copy)]
pub enum LineType {
    Solid,
    None,
}

#[derive(Clone, Copy)]
pub struct LineDesc {
    pub color: Color,
    pub marker: Marker,
    pub line_type: LineType,
}

impl Default for LineDesc {
    fn default() -> Self {
        LineDesc {
            color: GREEN,
            marker: Marker::None,
            line_type: LineType::Solid,
        }
    }
}

impl From<&str> for LineDesc {
    fn from(desc: &str) -> LineDesc {
        let mut color = GREEN;
        let mut line_type = LineType::None;
        let mut marker = Marker::None;

        let mut radius = 5.;

        for char in desc.chars() {
            match char {
                'r' => color = RED,
                'g' => color = GREEN,
                'b' => color = BLUE,
                'y' => color = YELLOW,
                //cyan
                'c' => color = Color::new(0., 1., 1., 1.),
                'o' => marker = Marker::Circle(radius),
                '-' => line_type = LineType::Solid,
                '1'..='9' => radius = char.to_digit(10).unwrap() as f32,
                _ => {}
            }
        }
        if marker == Marker::None {
            line_type = LineType::Solid
        }
        LineDesc {
            color,
            marker,
            line_type,
        }
    }
}
