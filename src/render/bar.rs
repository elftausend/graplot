use litequad::prelude::{clear_background, next_frame, WHITE, draw_line, GRAY, screen_width, screen_height, BLACK, draw_rectangle, draw_text};

use crate::Bar;

use super::COORD_THICKNESS;



pub async fn run(bar: Bar) {
    loop {
        clear_background(WHITE);

        let x_level = screen_height()-40.;

        // x axis
        draw_line(0., x_level, screen_width(), x_level, COORD_THICKNESS, GRAY);

        // y axis
        draw_line(40., 0., 40., screen_height(), COORD_THICKNESS, GRAY);

        //draw_rectangle(10., 40., 10.,40., GRAY);

        let mut line_x = 40.;
        let mut bar_x = 40. + 20.;
        for (bar_entity, y) in bar.bars.iter().zip(&bar.ys) {
            
    
            draw_rectangle(bar_x, x_level-bar.desc.spacing_y, bar_entity.width, bar.desc.spacing_y, bar_entity.color);
            
            line_x += bar_entity.width / 2. + 20.;
            draw_line(line_x, x_level-6., line_x, x_level+6., COORD_THICKNESS, BLACK);

            let text_pos = line_x - bar_entity.label.len() as f32 * (10. / 2.) + COORD_THICKNESS;

            draw_text(&bar_entity.label, text_pos, x_level + 20. / 2. + 8., 20., BLACK);
            line_x += bar_entity.width / 2.;
            bar_x += bar_entity.width + 20.;
        }

        next_frame().await;
    }
}