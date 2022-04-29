use litequad::prelude::{clear_background, next_frame, WHITE, draw_line, GRAY, screen_width, screen_height, BLACK, draw_rectangle, draw_text, DARKGRAY};
use crate::{Bar, max, max_display, get_font_size_y, get_steps, min, divs};
use super::COORD_THICKNESS;

const DISTANCE: f32 = 80.;

pub async fn run(bar: Bar, _min_y: f64) {

    let mut max_y = max(&bar.ys);
    max_y = max_display(max_y);

    let min_y = min(&bar.ys).min(0.);
    
    let y_font_size = get_font_size_y(max_y);
    let y_half_font = y_font_size / 2.;

    let steps_y = get_steps(max_y-min_y, bar.desc.min_steps_y.into());
    let step_y = (max_y - min_y) / steps_y;

    let ys = divs(&bar.ys, step_y);

    loop {
        clear_background(WHITE);

        for (idx, val) in (step_y as i128..=max_y as i128)
                .step_by(step_y as usize)
                .enumerate()
            {
                let y = (screen_height() - bar.desc.spacing_y * idx as f32) - bar.desc.spacing_y * 2.;

                let text = format!("{}", val);
                let move_away = text.len();

                draw_text(
                    &text,
                    DISTANCE - 5. - (y_half_font * move_away as f32),
                    y + (y_half_font / 2.),
                    y_font_size,
                    BLACK,
                );

                draw_line(DISTANCE - 4., y, DISTANCE + 4., y, 3., DARKGRAY);
            }

        let x_level = screen_height()-40.;

        let mut line_x = DISTANCE;
        let mut bar_x = DISTANCE + 20.;
        for (bar_entity, y) in bar.bars.iter().zip(&ys) {
            let y = *y as f32;
            draw_rectangle(bar_x, x_level-bar.desc.spacing_y * y, bar_entity.width, bar.desc.spacing_y * y, bar_entity.color);
            
            line_x += bar_entity.width / 2. + 20.;
            draw_line(line_x, x_level-6., line_x, x_level+6., COORD_THICKNESS, BLACK);

            let text_pos = line_x - bar_entity.label.len() as f32 * (10. / 2.) + COORD_THICKNESS;

            draw_text(&bar_entity.label, text_pos, x_level + 20. / 2. + 8., 20., BLACK);
            line_x += bar_entity.width / 2.;
            bar_x += bar_entity.width + 20.;
        }

        // x axis
        draw_line(0., x_level, screen_width(), x_level, COORD_THICKNESS, GRAY);

        // y axis
        draw_line(DISTANCE, 0., DISTANCE, screen_height(), COORD_THICKNESS, GRAY);
        

        next_frame().await;
    }
}