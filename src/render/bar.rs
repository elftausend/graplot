use litequad::prelude::{clear_background, next_frame, WHITE, draw_line, GRAY, screen_width, screen_height};

use super::COORD_THICKNESS;



pub async fn run() {
    loop {
        clear_background(WHITE);

        //x axis
        draw_line(0., screen_height()-40., screen_width(), screen_height()-40., COORD_THICKNESS, GRAY);

        next_frame().await;
    }
}