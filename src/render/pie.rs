use litequad::prelude::{clear_background, WHITE, next_frame, draw_poly_angle, Color, PINK, MAGENTA, DARKBROWN, DARKGREEN, DARKPURPLE, ORANGE, DARKGRAY, GOLD, GRAY, LIME, SKYBLUE, PURPLE, BROWN, BLUE, YELLOW, GREEN, RED, screen_height, screen_width};
use crate::Pie;

const COLOR_ARRAY: [Color; 17] = [RED, GREEN, ORANGE, BLUE, PINK, MAGENTA, BROWN, PURPLE, SKYBLUE, LIME, GRAY, DARKGREEN, DARKBROWN, GOLD, DARKPURPLE, YELLOW, DARKGRAY];

pub async fn run(pie: Pie) {

    loop {
        clear_background(WHITE);

        let mut angle = 0.;
        let mut color_idx = 0;
        for segment in &pie.segs {
            if color_idx == 17 {
                color_idx = 0;
            }

            draw_poly_angle(screen_width() / 2., screen_height() / 2., 30, 110., 360.-angle, segment.color.unwrap_or(COLOR_ARRAY[color_idx]));
            angle += (segment.percentage * 3.6) as f32;
            color_idx += 1;
        }

        next_frame().await;
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}