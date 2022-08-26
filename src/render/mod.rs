pub mod plot;
pub mod bar;
pub mod pie;
pub mod plot3d;
pub mod graph;

use litequad::prelude::{Color, PINK, MAGENTA, DARKBROWN, DARKGREEN, DARKPURPLE, ORANGE, DARKGRAY, GOLD, GRAY, LIME, SKYBLUE, PURPLE, BROWN, BLUE, YELLOW, GREEN, RED};

const COLOR_ARRAY: [Color; 17] = [RED, GREEN, ORANGE, BLUE, PINK, MAGENTA, BROWN, PURPLE, SKYBLUE, LIME, GRAY, DARKGREEN, DARKBROWN, GOLD, DARKPURPLE, YELLOW, DARKGRAY];

const TITLE_SIZE: f32 = 37.;
const YLABEL_SIZE: f32 = 29.;
//const XLABEL_SIZE: f32 = 29.;
pub const FONT_SIZE: f32 = 24.; //27.
const COORD_THICKNESS: f32 = 2.;
const DISTANCE_X_AXIS: f32 = 70.;


