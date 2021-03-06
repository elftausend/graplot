use std::fmt::Display;

use litequad::prelude::{clear_background, next_frame, WHITE, draw_line, GRAY, screen_width, screen_height, BLACK, draw_rectangle, draw_text, DARKGRAY, draw_text_rot, is_key_pressed, KeyCode};
use crate::{Bar, get_font_size_y, min, divs, count_inv_tens, negative};
use super::{COORD_THICKNESS, YLABEL_SIZE, TITLE_SIZE, DISTANCE_X_AXIS};
const DISTANCE: f32 = 80.;

pub async fn run(bar: Bar, _min_y: f64, max_y: f64, steps_y: f64) {
    let min_y = min(&bar.ys);
    
    let y_font_size = get_font_size_y(max_y);
    let y_half_font = y_font_size / 2.;

    let step_y = (max_y) / steps_y;

    let ys = divs(&bar.ys, step_y);

    if min_y >= 0. {
        return positive_only(bar, step_y, y_half_font, y_font_size, max_y, ys).await;
    }
    if negative(&ys) {
        return negative_only(bar, step_y, y_half_font, y_font_size, max_y, ys).await;
    }
    positive_negative(bar, step_y, y_half_font, y_font_size, max_y, ys).await
}

pub async fn draw_line_with_text<T: Display>(y: f32, val: T, y_half_font: f32, y_font_size: f32) {
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

pub async fn draw_bars(x_level: f32, ys: &[f64], bar: &Bar) {
    let mut line_x = DISTANCE;
    let mut bar_x = DISTANCE + 20.;
    for (bar_entity, y) in bar.bars.iter().zip(ys) {
        let y = *y as f32;
        draw_rectangle(bar_x, x_level-bar.desc.spacing_y * y, bar_entity.width, bar.desc.spacing_y * y, bar_entity.color);
        
        line_x += bar_entity.width / 2. + 20.;
        draw_line(line_x, x_level-6., line_x, x_level+6., COORD_THICKNESS, BLACK);

        let text_pos = line_x - bar_entity.label.len() as f32 * (10. / 2.) + COORD_THICKNESS;

        draw_text(&bar_entity.label, text_pos, x_level + 20. / 2. + 8., 20., BLACK);
        line_x += bar_entity.width / 2.;
        bar_x += bar_entity.width + 20.;
    }
}

pub async fn negative_only(bar: Bar, step_y: f64, y_half_font: f32, y_font_size: f32, max_y: f64, ys: Vec<f64>) {
    loop {
        clear_background(WHITE);

        for (idx, char) in bar.axis_desc.y_label.chars().into_iter().enumerate() {
            draw_text_rot(&char.to_string(), 7., screen_height() / 2. - (YLABEL_SIZE / 2. * idx as f32) , YLABEL_SIZE, BLACK, -std::f32::consts::PI / 2.,);
        }

        draw_text(&bar.axis_desc.title, screen_width() / 2., DISTANCE_X_AXIS / 3. + 7., TITLE_SIZE, BLACK);
        draw_text(&bar.axis_desc.x_label, screen_width() / 2., screen_height() - DISTANCE_X_AXIS / 3., YLABEL_SIZE, BLACK);

        // x axis
        draw_line(0., DISTANCE_X_AXIS, screen_width(), DISTANCE_X_AXIS, COORD_THICKNESS, GRAY);

        // y axis
        draw_line(DISTANCE, 0., DISTANCE, screen_height(), COORD_THICKNESS, GRAY);

        if step_y > 1. {
            for (idx, val) in (step_y as i128..=max_y as i128)
                .step_by(step_y as usize)
                .enumerate()
            {
                let y = DISTANCE_X_AXIS + bar.desc.spacing_y * idx as f32 + bar.desc.spacing_y;
                draw_line_with_text(y, -val, y_half_font, y_font_size).await;
            }
        } else {
            let tens_step = count_inv_tens(step_y);

            let max_y = max_y * tens_step as f64;
            let step_y = step_y * tens_step as f64;

            for (idx, val) in (step_y as i128..=max_y as i128)
                .step_by(step_y as usize)
                .enumerate()
            {
                let y = (screen_height() / 2.) + bar.desc.spacing_y * idx as f32 + bar.desc.spacing_y;
                draw_line_with_text(y, -val as f64 / tens_step as f64, y_half_font, y_font_size).await;
            }
        }
        
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        draw_bars(DISTANCE_X_AXIS, &ys, &bar).await;
        next_frame().await;
        std::thread::sleep(std::time::Duration::from_millis(16));        
    }
}

pub async fn positive_negative(bar: Bar, step_y: f64, y_half_font: f32, y_font_size: f32, max_y: f64, ys: Vec<f64>) {
    loop {
        clear_background(WHITE);

        for (idx, char) in bar.axis_desc.y_label.chars().into_iter().enumerate() {
            draw_text_rot(&char.to_string(), 7., screen_height() / 2. - (YLABEL_SIZE / 2. * idx as f32) , YLABEL_SIZE, BLACK, -std::f32::consts::PI / 2.,);
        }

        draw_text(&bar.axis_desc.title, screen_width() / 2., DISTANCE_X_AXIS / 3. + 7., TITLE_SIZE, BLACK);
        draw_text(&bar.axis_desc.x_label, screen_width() / 2., screen_height() - DISTANCE_X_AXIS / 3., YLABEL_SIZE, BLACK);

        // x axis
        draw_line(0., screen_height() / 2., screen_width(), screen_height() / 2., COORD_THICKNESS, GRAY);

        // y axis
        draw_line(DISTANCE, 0., DISTANCE, screen_height(), COORD_THICKNESS, GRAY);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if step_y > 1. {
            for (idx, val) in (step_y as i128..=max_y as i128)
                .step_by(step_y as usize)
                .enumerate()
            {
                let y = (screen_height() / 2.) - bar.desc.spacing_y * idx as f32 - bar.desc.spacing_y;
                draw_line_with_text(y, val, y_half_font, y_font_size).await;
            }
        } else {
            let tens_step = count_inv_tens(step_y);

            let max_y = max_y * tens_step as f64;
            let step_y = step_y * tens_step as f64;

            for (idx, val) in (step_y as i128..=max_y as i128)
                .step_by(step_y as usize)
                .enumerate()
            {
                let y = (screen_height() / 2.) - bar.desc.spacing_y * idx as f32 - bar.desc.spacing_y;
                draw_line_with_text(y, val as f64 / tens_step as f64, y_half_font, y_font_size).await;
            }
        }

        if step_y > 1. {
            for (idx, val) in (step_y as i128..=max_y as i128)
                .step_by(step_y as usize)
                .enumerate()
            {
                let y = (screen_height() / 2.) + bar.desc.spacing_y * idx as f32 + bar.desc.spacing_y;
                draw_line_with_text(y, -val, y_half_font, y_font_size).await;
            }
        } else {
            let tens_step = count_inv_tens(step_y);

            let max_y = max_y * tens_step as f64;
            let step_y = step_y * tens_step as f64;

            for (idx, val) in (step_y as i128..=max_y as i128)
                .step_by(step_y as usize)
                .enumerate()
            {
                let y = (screen_height() / 2.) + bar.desc.spacing_y * idx as f32 + bar.desc.spacing_y;
                draw_line_with_text(y, -val as f64 / tens_step as f64, y_half_font, y_font_size).await;
            }
        }
        draw_bars(screen_height() / 2., &ys, &bar).await;
        
        next_frame().await;
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
    
}

pub async fn positive_only(bar: Bar, step_y: f64, y_half_font: f32, y_font_size: f32, max_y: f64, ys: Vec<f64>) {
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
                draw_line_with_text(y, val, y_half_font, y_font_size).await;
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
                draw_line_with_text(y, val as f64 / tens_step as f64, y_half_font, y_font_size).await;
            }
        }
        
        let x_level = screen_height()-DISTANCE_X_AXIS;
        draw_bars(x_level, &ys, &bar).await;

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