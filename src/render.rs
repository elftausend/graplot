use macroquad::prelude::*;

use crate::{LineType, Marker, Matrix, Plot};

const FONT_SIZE: f32 = 24.; //27.
const COORD_THICKNESS: f32 = 2.;

pub fn max(arr: &[f64]) -> f64 {
    let mut max = arr[0];
    for value in arr {
        if value > &max {
            max = *value;
        }
    }
    max
}

pub fn count_tens(mut num: f64) -> u128 {
    let mut count = 0;
    while num > 10. {
        num /= 10.;
        count += 1;
    }
    10u128.pow(count)
}

pub fn count_inv_tens(mut num: f64) -> u128 {
    let mut count = 1;
    while num.round() == 0. {
        num *= 10.;
        count += 1;
    }
    10u128.pow(count)
}

pub fn divs(lhs: &[f64], rhs: f64) -> Vec<f64> {
    let mut out = Vec::with_capacity(lhs.len());
    for val in lhs.iter() {
        out.push(val / rhs);
    }
    out
}

fn get_font_size_x(max: f64) -> f32 {
    let a = if max >= 3. {
        count_tens(max) * 10
    } else {
        count_inv_tens(max) * 12
    };

    let a = (a as f64).log10() as f32;
    FONT_SIZE - (2.9 * a)
}

fn get_font_size_y(max: f64) -> f32 {
    let a = if max >= 3. {
        count_tens(max) * 10
    } else {
        count_inv_tens(max) * 1000
    };

    let a = (a as f64).log10() as f32;
    FONT_SIZE - (1. * a)
}

pub fn get_steps(max: f64) -> f64 {
    let mut steps = 4.;

    if max >= 4. {
        while max % steps != 0. {
            steps += 1.;
        }
    }

    steps
}

fn max_display(max: f64) -> f64 {
    if max == 0. {
        return 1.;
    }
    if max >= 2. {
        let tens = count_tens(max);
        ((max / tens as f64 / 2.).round() * tens as f64) * 2.
    } else {
        let tens = count_inv_tens(max);
        ((max * tens as f64 / 2.).round() / tens as f64) * 2.
    }
}

pub fn max_matrix(mat: &Matrix) -> f64 {
    let mut max = mat[0][0].abs();
    for x in mat {
        for x in x {
            let x = x.abs();
            if x > max {
                max = x;
            }
        }
    }
    max
}

pub async fn run(plot: Plot) {
    let mut max_x = max_matrix(&plot.xs);
    //let mut max_x = max(&maxed);

    max_x = max_display(max_x);

    let x_font_size = get_font_size_x(max_x);

    let steps = get_steps(max_x);
    let step_x = max_x / steps;

    let start_x = step_x;

    //let maxed = plot.ys.iter().map(|y| y.abs()).collect::<Vec<f64>>();
    //let mut max_y = max(&maxed);
    let mut max_y = max_matrix(&plot.ys);
    max_y = max_display(max_y);

    let y_font_size = get_font_size_y(max_y);

    let steps_y = get_steps(max_y);
    let step_y = max_y / steps_y;

    let start_y = step_y;

    loop {
        clear_background(WHITE);

        let half_height = screen_height() / 2.;
        let half_width = screen_width() / 2.;

        if !plot.axis_desc.title.is_empty() {
            draw_text(
                &plot.axis_desc.title,
                half_width - plot.axis_desc.title.len() as f32 * (15. / 2.) + 15.,
                half_height - 40. * steps_y as f32 - 20.,
                30.,
                BLACK,
            );
        }

        // y-axis
        draw_line(
            half_width,
            half_height - 40. * steps_y as f32,
            half_width,
            screen_height(),
            COORD_THICKNESS,
            GRAY,
        );

        // x-axis
        draw_line(
            0.0,
            half_height,
            screen_width(),
            half_height,
            COORD_THICKNESS,
            GRAY,
        );

        for (idx, xs) in plot.xs.iter().enumerate() {
            let xs = divs(xs, step_x);
            let ys = divs(&plot.ys[idx], step_y);

            let line_desc = &plot.line_desc[idx];

            let mut coords = Vec::new();

            for i in 0..xs.len() {
                let x = xs[i] as f32;
                let y = ys[i] as f32;

                let x = half_width + 40. * x;
                let y = half_height - 40. * y;

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
                let y = (half_height - 40. * idx as f32) - 40.;
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
                let y = (half_height - 40. * idx as f32) - 40.;
                let text = format!("{}", val as f64 / tens_start as f64);
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
        }

        if step_y > 1. {
            for (idx, val) in (start_y as i128..=max_y as i128)
                .step_by(step_y as usize)
                .enumerate()
            {
                let y = (half_height - 40. * -(idx as f32)) + 40.;
                let text = format!("{}", -val);
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
                let y = (half_height - 40. * -(idx as f32)) + 40.;
                let text = format!("{}", -val as f64 / tens_start as f64);
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
        }

        if step_x > 1. {
            for (idx, val) in (start_x as i128..=max_x as i128)
                .step_by(step_x as usize)
                .enumerate()
            {
                let x = (half_width + 40. * idx as f32) + 40.;

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
                let x = (half_width + 40. * idx as f32) + 40.;

                let text = format!("{}", val as f64 / tens_start as f64);

                draw_text(
                    &text,
                    x - text.len() as f32 * (x_half_font / 2.),
                    half_height + x_half_font + 7.,
                    x_font_size,
                    BLACK,
                );
                draw_line(x, half_height + 4., x, half_height - 4., 3., DARKGRAY);
            }
        }

        if step_x > 1. {
            for (idx, val) in (start_x as i128..=max_x as i128)
                .step_by(step_x as usize)
                .enumerate()
            {
                let x = (half_width + 40. * -(idx as f32)) - 40.;

                let text = format!("{}", -val);

                draw_text(
                    &text,
                    x - text.len() as f32 * (x_half_font / 2.),
                    half_height + x_half_font + 7.,
                    x_font_size,
                    BLACK,
                );
                draw_line(x, half_height + 4., x, half_height - 4., 3., DARKGRAY);
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
                let x = (half_width + 40. * -(idx as f32)) - 40.;

                let text = format!("{}", -val as f64 / tens_start as f64);

                draw_text(
                    &text,
                    x - text.len() as f32 * (x_half_font / 2.),
                    half_height + x_half_font + 7.,
                    x_font_size,
                    BLACK,
                );
                draw_line(x, half_height + 4., x, half_height - 4., 3., DARKGRAY);
            }
        }

        next_frame().await;
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
