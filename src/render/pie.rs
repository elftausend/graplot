use litequad::prelude::{clear_background, WHITE, next_frame, draw_poly_angle, screen_height, screen_width, draw_text, BLACK, is_key_pressed, KeyCode};
use crate::Pie;

use super::{TITLE_SIZE, DISTANCE_X_AXIS, COLOR_ARRAY};

pub async fn run(pie: Pie) {

    loop {
        clear_background(WHITE);

        draw_text(&pie.title, screen_width() / 2. - (pie.title.len() as f32* TITLE_SIZE / 4.), DISTANCE_X_AXIS / 3. + 7., TITLE_SIZE, BLACK);

        let mut angle = 0.;
        for (color_idx, segment) in pie.segs.iter().enumerate() {

            draw_poly_angle(screen_width() / 2., screen_height() / 2., 30, pie.radius as f32, 360.-angle, segment.color.unwrap_or(COLOR_ARRAY[color_idx % COLOR_ARRAY.len()]));

            let radians = (360f32-(angle + (segment.percentage * 3.6) as f32 / 2.)).to_radians();

            let mut add_x = 0.;
            if (360.-angle) > 126. && (360. - angle ) < 250. {
                add_x = segment.label.len() as f32 * (25. / 2.);
            }

            let rx = screen_width() / 2. + radians.cos() * (pie.radius as f32 + 10.);
            let ry = screen_height() / 2. + radians.sin() * (pie.radius as f32 + 16.);
        
            draw_text(&segment.label, rx - add_x, ry, 25., BLACK);

            angle += (segment.percentage * 3.6) as f32;
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        
        next_frame().await;
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}