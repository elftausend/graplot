use litequad::prelude::{clear_background, next_frame, WHITE, draw_line, GRAY, screen_width, screen_height, BLACK, draw_rectangle, draw_text, DARKGRAY, draw_text_rot, is_key_pressed, KeyCode};
use crate::{Bar, max, max_display, get_font_size_y, get_steps, min, divs, count_inv_tens};
use super::{COORD_THICKNESS, YLABEL_SIZE, TITLE_SIZE, DISTANCE_X_AXIS};

const DISTANCE: f32 = 80.;

pub async fn run(bar: Bar, _min_y: f64) {

    let mut max_y = max(&bar.ys);
    max_y = max_display(max_y, false);

    let min_y = min(&bar.ys).min(0.);
    
    let y_font_size = get_font_size_y(max_y);
    let y_half_font = y_font_size / 2.;

    let steps_y = get_steps(max_y-min_y, bar.desc.min_steps_y.into());
    let step_y = (max_y - min_y) / steps_y;

    let ys = divs(&bar.ys, step_y);

    loop {
        clear_background(WHITE);

        for (idx, char) in bar.axis_desc.y_label.chars().into_iter().enumerate() {
            draw_text_rot(&char.to_string(), 7., screen_height() / 2. - (YLABEL_SIZE / 2. * idx as f32) , YLABEL_SIZE, BLACK, -std::f32::consts::PI / 2.,);
        }
    
        draw_text(&bar.axis_desc.title, screen_width() / 2., DISTANCE_X_AXIS / 3. + 7., TITLE_SIZE, BLACK);
        draw_text(&bar.axis_desc.x_label, screen_width() / 2., screen_height() - DISTANCE_X_AXIS / 3., YLABEL_SIZE, BLACK);

        if step_y > 1. {
            for (idx, val) in (step_y as i128..=max_y as i128)
                .step_by(step_y as usize)
                .enumerate()
            {
                let y = (screen_height() - bar.desc.spacing_y * idx as f32) - DISTANCE_X_AXIS - bar.desc.spacing_y;

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

        } else {
            let tens_step = count_inv_tens(step_y);

            let max_y = max_y * tens_step as f64;
            let step_y = step_y * tens_step as f64;

            for (idx, val) in (step_y as i128..=max_y as i128)
                .step_by(step_y as usize)
                .enumerate()
            {
                let y = (screen_height() - bar.desc.spacing_y * idx as f32) - DISTANCE_X_AXIS - bar.desc.spacing_y;

                let text = format!("{}", val as f64 / tens_step as f64);
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
        }
        
        let x_level = screen_height()-DISTANCE_X_AXIS;

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
        
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}