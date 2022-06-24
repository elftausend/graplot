use litequad::prelude::*;

use crate::{LineType, Marker, Plot, max_matrix, max_display, get_font_size_y, get_steps, get_font_size_x, divs, sub, count_inv_tens};

use super::{TITLE_SIZE, COORD_THICKNESS, YLABEL_SIZE};

pub async fn run(plot: Plot, min_y: f64, other_scaling: bool) {
    let spacing_x = plot.desc.spacing_x;
    let spacing_y = plot.desc.spacing_y;

    let mut max_x = max_matrix(&plot.xs);
    //let mut max_x = max(&maxed);

    max_x = max_display(max_x, other_scaling);

    let x_font_size = get_font_size_x(max_x);

    let steps = get_steps(max_x, plot.desc.min_steps_x.into());
    let step_x = max_x / steps;

    let start_x = step_x;

    //let maxed = plot.ys.iter().map(|y| y.abs()).collect::<Vec<f64>>();
    //let mut max_y = max(&maxed);

    let mut max_y = max_matrix(&plot.ys);
    max_y = max_display(max_y, other_scaling);

    let y_font_size = get_font_size_y(max_y);

    let steps_y = get_steps(max_y, plot.desc.min_steps_y.into());
    let step_y = (max_y) / steps_y;

    let start_y = if min_y > 0. { min_y } else { step_y };
    //let start_y = step_y;

    loop {
        clear_background(WHITE);

        let half_height = screen_height() / 2.;
        let half_width = screen_width() / 2.;

        if !plot.axis_desc.title.is_empty() {
            let len = plot.axis_desc.title.len() as f32 * (TITLE_SIZE / 4. - COORD_THICKNESS / 2.);
            draw_text(
                &plot.axis_desc.title,
                half_width - len,
                half_height - spacing_y * steps_y as f32 - 50.,
                TITLE_SIZE,
                BLACK,
            );
        }

        if !plot.axis_desc.y_label.is_empty() {
            let len =
                plot.axis_desc.y_label.len() as f32 * (YLABEL_SIZE / 4. - COORD_THICKNESS / 2.);
            draw_text(
                &plot.axis_desc.y_label,
                half_width - len,
                half_height - spacing_y * steps_y as f32 - 20.,
                YLABEL_SIZE,
                BLACK,
            );
        }

        if !plot.axis_desc.x_label.is_empty() {
            //let len = plot.axis_desc.y_label.len() as f32 * (YLABEL_SIZE / 4. - COORD_THICKNESS / 2.);
            for (idx, char) in plot.axis_desc.x_label.chars().into_iter().enumerate() {
                let adding = if char == 'i' { 3. } else { 0. };

                draw_text_rot(
                    &format!("{char}"),
                    half_width + spacing_x * steps as f32 + 20. + adding,
                    half_height
                        - plot.axis_desc.x_label.len() as f32
                            * (YLABEL_SIZE / 4. - COORD_THICKNESS / 2.)
                        + idx as f32 * YLABEL_SIZE / 2.,
                    YLABEL_SIZE,
                    BLACK,
                    std::f32::consts::PI / 2.,
                );
            }
        }

        let (subtract_x, subtract_y) = if !plot.axis_desc.y_label.is_empty()
            || !plot.axis_desc.x_label.is_empty()
            || !plot.axis_desc.title.is_empty()
        {
            (spacing_x * steps as f32, (spacing_y * steps_y as f32))
        } else {
            (half_width, half_height)
        };

        // y-axis
        draw_line(
            half_width,
            half_height - subtract_y,
            half_width,
            screen_height(),
            COORD_THICKNESS,
            GRAY,
        );

        // x-axis
        draw_line(
            0.0,
            half_height,
            half_width + subtract_x,
            half_height,
            COORD_THICKNESS,
            GRAY,
        );

        for (idx, xs) in plot.xs.iter().enumerate() {
            let xs = divs(xs, step_x);

            let move_up = if min_y > 0. {step_y} else {0.};
            let ys = sub(&plot.ys[idx], min_y - move_up);
            let ys = divs(&ys, step_y);
            

            let line_desc = &plot.line_desc[idx];

            let mut coords = Vec::new();

            for i in 0..xs.len() {
                let x = xs[i] as f32;
                let y = ys[i] as f32;

                let x = half_width + spacing_x * x;
                let y = half_height - spacing_y * y;

                coords.push((x, y));

                match line_desc.marker {
                    Marker::Circle(r) => draw_circle(x, y, r, line_desc.color),
                    Marker::None => {}
                }

                if coords.len() >= 2 {
                    match line_desc.line_type {
                        LineType::Solid => draw_line(
                            coords[0].0,
                            coords[0].1,
                            coords[1].0,
                            coords[1].1,
                            3.,
                            line_desc.color,
                        ),
                        LineType::None => {}
                    }
                    coords.remove(0);
                }
            }
        }

        let y_half_font = y_font_size / 2.;

        let x_half_font = x_font_size / 2.;

        //draw_text("0", half_width - 13.5, half_height + 17., 14., BLACK);

        if step_y > 1. {
            for (idx, val) in (start_y as i128..=max_y as i128)
                .step_by(step_y as usize)
                .enumerate()
            {
                let y = (half_height - spacing_y * idx as f32) - spacing_y;
                axis_desc_y(val , y, half_width, y_half_font, y_font_size);
            }
        } else {
            let tens_start = count_inv_tens(start_y);
            let tens_step = count_inv_tens(step_y);

            let start_y = start_y * tens_start as f64;
            let max_y = max_y * tens_start as f64;
            let step_y = step_y * tens_step as f64;

            for (idx, val) in (start_y as i128..=max_y as i128)
                .step_by(step_y as usize)
                .enumerate()
            {
                let y = (half_height - spacing_y * idx as f32) - spacing_y;
                axis_desc_y(val as f64 / tens_start as f64, y, half_width, y_half_font, y_font_size);
            }
        }

        if step_y > 1. {
            for (idx, val) in (start_y as i128..=max_y as i128)
                .step_by(step_y as usize)
                .enumerate()
            {
                let y = (half_height - spacing_y * -(idx as f32)) + spacing_y;
                axis_desc_y(-val , y, half_width, y_half_font, y_font_size);
            }
        } else {
            let tens_start = count_inv_tens(start_y);

            let tens_step = count_inv_tens(step_y);

            let start_y = start_y * tens_start as f64;
            let max_y = max_y * tens_start as f64;
            let step_y = step_y * tens_step as f64;

            for (idx, val) in (start_y as i128..=max_y as i128)
                .step_by(step_y as usize)
                .enumerate()
            {
                let y = (half_height - spacing_y * -(idx as f32)) + spacing_y;
                axis_desc_y(-val as f64 / tens_start as f64 , y, half_width, y_half_font, y_font_size);
            }
        }

        if step_x > 1. {
            for (idx, val) in (start_x as i128..=max_x as i128)
                .step_by(step_x as usize)
                .enumerate()
            {
                let x = (half_width + spacing_x * idx as f32) + spacing_x;
                axis_desc_x(val, x, x_half_font, x_font_size, half_height);
            }
        } else {
            let tens_start = count_inv_tens(start_x);
            let tens_step = count_inv_tens(step_x);

            let start_x = start_x * tens_start as f64;
            let max_x = max_x * tens_start as f64;
            let step_x = step_x * tens_step as f64;

            for (idx, val) in (start_x as i128..=max_x as i128)
                .step_by(step_x as usize)
                .enumerate()
            {
                let x = (half_width + spacing_x * idx as f32) + spacing_x;
                axis_desc_x(val as f64 / tens_start as f64, x, x_half_font, x_font_size, half_height);
            }
        }

        if step_x > 1. {
            for (idx, val) in (start_x as i128..=max_x as i128)
                .step_by(step_x as usize)
                .enumerate()
            {
                let x = (half_width + spacing_x * -(idx as f32)) - spacing_x;
                axis_desc_x(-val, x, x_half_font, x_font_size, half_height);
            }
        } else {
            let tens_start = count_inv_tens(start_x);
            let tens_step = count_inv_tens(step_x);

            let start_x = start_x * tens_start as f64;
            let max_x = max_x * tens_start as f64;
            let step_x = step_x * tens_step as f64;

            for (idx, val) in (start_x as i128..=max_x as i128)
                .step_by(step_x as usize)
                .enumerate()
            {
                let x = (half_width + spacing_x * -(idx as f32)) - spacing_x;
                axis_desc_x(-val as f64 / tens_start as f64, x, x_half_font, x_font_size, half_height);
            }
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}


fn axis_desc_y<T: std::fmt::Display>(val: T, y: f32, half_width: f32, y_half_font: f32, y_font_size: f32) {
    let text = format!("{}", val);
    let move_away = text.len();

    draw_text(
        &text,
        half_width - 5. - (y_half_font * move_away as f32),
        y + (y_half_font / 2.),
        y_font_size,
        BLACK,
    );
    draw_line(half_width - 4., y, half_width + 4., y, 3., DARKGRAY);
}

fn axis_desc_x<T: std::fmt::Display>(val: T, x: f32, x_half_font: f32, x_font_size: f32, half_height: f32) {
    let text = format!("{}", val);

    draw_text(
        &text,
        x - text.len() as f32 * (x_half_font / 2.),
        half_height + x_half_font + 7.,
        x_font_size,
        BLACK,
    );
    draw_line(x, half_height + 4., x, half_height - 4., 3., DARKGRAY);
}