use litequad::prelude::{clear_background, WHITE, next_frame, draw_poly_angle, Color, PINK, MAGENTA, DARKBROWN, DARKGREEN, DARKPURPLE, ORANGE, DARKGRAY, GOLD, GRAY, LIME, SKYBLUE, PURPLE, BROWN, BLUE, YELLOW, GREEN, RED, screen_height, screen_width, draw_text, BLACK};
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

            draw_poly_angle(screen_width() / 2., screen_height() / 2., 30, pie.radius as f32, 360.-angle, segment.color.unwrap_or(COLOR_ARRAY[color_idx]));

            let radians = (360f32-(angle + (segment.percentage * 3.6) as f32 / 2.)).to_radians();

            let mut add = 0.;
            if (360.-angle) > 180. && (360. - angle ) < 270. {
                add = 40.;
                println!("color_idx: {color_idx}");
                println!("angle: {angle}");
            }

            let rx = screen_width() / 2. + radians.cos() * pie.radius as f32;
            let ry = screen_height() / 2. + radians.sin() * pie.radius as f32;
            
            println!("add: {add}");
            draw_text(&format!("test{color_idx}", ), rx - add, ry, 25., BLACK);

            angle += (segment.percentage * 3.6) as f32;
            color_idx += 1;
        }
        next_frame().await;
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}