use macroquad::prelude::*;

const FONT_SIZE: f32 = 24.; //27.
const COORD_THICKNESS: f32 = 2.;


pub fn min(arr: &[f32]) -> f32 {
    let mut min = arr[0];
    for value in arr {
        if value < &min {
            min = *value;
        }
    }
    min
}

pub fn max(arr: &[f64]) -> f64 {
    let mut max = arr[0];
    for value in arr {
        if value > &max {
            max = *value;
        }
    }
    max
}

pub fn count_tens(mut num: f64) -> usize {
    let mut count = 0;
    while num > 10. {
        num /= 10.;
        count += 1;
    }
    10usize.pow(count)
}

pub fn count_inv_tens(mut num: f64) -> usize {
    let mut count = 0;
    while num % 1. != 0. {
        num *= 10.;
        count += 1;
    }
    10usize.pow(count)
}

pub fn divs(lhs: &[f64], rhs: f64) -> Vec<f64> {
    let mut out = Vec::with_capacity(lhs.len());
    for val in lhs.iter() {
        out.push(val / rhs);
    }
    out
}

pub fn round(n: i32) -> i32 {
    let tens = count_tens(n as f64);
    ((n as f32 / tens as f32).round() as usize * tens) as i32
}

pub fn smaller_round(n: f64) -> f64 {
    let tens = count_inv_tens(n);
    round((n * tens as f64) as i32) as f64 / tens as f64
}

#[test]
fn test_round() {
    let a = 7340;
    let r = round(a);
    assert!(r == 7000);


    let a = 0.085;
    
    let tens = count_inv_tens(a);


    let r = round((a * tens as f64) as i32) as f64 / tens as f64;
    println!("r: {r}")
}

fn get_font_size(max: f64) -> f32 {
    let a = if max >= 3. {
        count_tens(max) * 10
    } else {
        count_inv_tens(max) * 1000
    };
    
    let a = (a as f64).log10() as i32;
    FONT_SIZE - (2.5 * a as f32)
}



pub async fn run(xs: Vec<f64>, ys: Vec<f64>, marker: String) {
    let mut max_x = max(&xs);
    
    if max_x >= 4. {
        max_x = round((max_x / 2.) as i32) as f64 * 2.;
    } else {
        max_x = smaller_round(max_x / 2.) * 2.;
    }

    let x_font_size = get_font_size(max_x);
    //println!("max_x: {max_x}");
    
    let steps = 4.;

    let step_x = max_x / steps;

    let start_x = step_x;

    let xs = divs(&xs, step_x);


    let mut max_y = max(&ys);
    
    if max_y >= 4. {
        max_y = round((max_y / 2.) as i32) as f64 * 2.;
    } else {
        max_y = smaller_round(max_y / 2.) * 2.;
    }

    let y_font_size = get_font_size(max_y);

    let steps_y = 4.;

    let step_y = max_y / steps_y;

    let start_y = step_y;

    let ys = divs(&ys, step_y);


    //let xs = divs(xs, max_x);
    
    //let tens = count_tens(max_x) as i32;
    //let res = (max_x as i32 / tens) * tens;
    //println!("res: {:?}", res);
    

    loop {
        clear_background(WHITE);

        let half_height = screen_height() / 2.;
        let half_width = screen_width() / 2.;

        let y_half_font = y_font_size / 2.;

        let x_half_font = x_font_size / 2.;
        
        //draw_text("0", half_width - 13.5, half_height + 17., 14., BLACK);

        if step_y > 1. {
            for (idx, val) in (start_y as i32..=max_y as i32).step_by(step_y as usize).enumerate() {
                let y = (half_height - 40. * idx as f32) - 40.;
                let text = format!("{}", val);
                let move_away = text.len();
    
                draw_text(&text, half_width - 5. - (y_half_font * move_away as f32), y, y_font_size, BLACK);
                draw_line(half_width-4., y, half_width+4., y, 3., DARKGRAY);
            }
        } 
        else {
            
            let tens_start = count_inv_tens(start_y);
            
            let tens_step = count_inv_tens(step_y);

            let start_y = start_y * tens_start as f64;
            let max_y = max_y * tens_start as f64;
            let step_y = step_y * tens_step as f64;

            for (idx, val) in (start_y as i32..=max_y as i32).step_by(step_y as usize).enumerate() {
                let y = (half_height - 40. * idx as f32) - 40.;
                let text = format!("{}", val as f32 / tens_start as f32);
                let move_away = text.len();
    
                draw_text(&text, half_width - 5. - (y_half_font * move_away as f32), y + (y_half_font / 2.), y_font_size, BLACK);
                draw_line(half_width-4., y, half_width+4., y, 3., DARKGRAY);
            }
        }

        
        if step_y > 1. {
            for (idx, val) in (start_y as i32..=max_y as i32).step_by(step_y as usize).enumerate() {
                let y = (half_height - 40. * -(idx as f32)) + 40.;
                let text = format!("{}", -val);
                let move_away = text.len();
    
                draw_text(&text, half_width - 5. - (y_half_font * move_away as f32), y, y_font_size, BLACK);
                draw_line(half_width-4., y, half_width+4., y, 3., DARKGRAY);
            }
        } else {
            
            let tens_start = count_inv_tens(start_y);
            
            let tens_step = count_inv_tens(step_y);

            let start_y = start_y * tens_start as f64;
            let max_y = max_y * tens_start as f64;
            let step_y = step_y * tens_step as f64;

            for (idx, val) in (start_y as i32..=max_y as i32).step_by(step_y as usize).enumerate() {
                let y = (half_height - 40. * -(idx as f32)) + 40.;
                let text = format!("{}", -val as f32 / tens_start as f32);
                let move_away = text.len();
    
                draw_text(&text, half_width - 5. - (y_half_font * move_away as f32), y + (y_half_font / 2.), y_font_size, BLACK);
                draw_line(half_width-4., y, half_width+4., y, 3., DARKGRAY);
        
            }
        }

        if step_x > 1. {
            for (idx, val) in (start_x as i32..=max_x as i32).step_by(step_x as usize).enumerate() {
                let x = (half_width + 40. * idx as f32) + 40.;
    
                let text = format!("{}", val);
    
                draw_text(&text, x - text.len() as f32 * (x_half_font / 2.), half_height + x_half_font + 7., x_font_size, BLACK);
                draw_line(x, half_height + 4., x, half_height - 4., 3., DARKGRAY);    
            }
        } else {
            let tens_start = count_inv_tens(start_x);
            let tens_step = count_inv_tens(step_x);

            let start_x = start_x * tens_start as f64;
            let max_x = max_x * tens_start as f64;
            let step_x = step_x * tens_step as f64;

            for (idx, val) in (start_x as i32..=max_x as i32).step_by(step_x as usize).enumerate() {
                let x = (half_width + 40. * idx as f32) + 40.;
    
                let text = format!("{}", val as f32 / tens_start as f32);
    
                draw_text(&text, x - text.len() as f32 * (x_half_font / 2.), half_height + x_half_font + 7., x_font_size, BLACK);
                draw_line(x, half_height + 4., x, half_height - 4., 3., DARKGRAY);    
            }
        }
        
        if step_x > 1. {
            for (idx, val) in (start_x as i32..=max_x as i32).step_by(step_x as usize).enumerate() {
                let x = (half_width + 40. * -(idx as f32)) - 40.;
    
                let text = format!("{}", -val);
    
                draw_text(&text, x - text.len() as f32 * (x_half_font / 2.), half_height + x_half_font + 7., x_font_size, BLACK);
                draw_line(x, half_height + 4., x, half_height - 4., 3., DARKGRAY);    
            }
        } else {
            let tens_start = count_inv_tens(start_x);
            let tens_step = count_inv_tens(step_x);

            let start_x = start_x * tens_start as f64;
            let max_x = max_x * tens_start as f64;
            let step_x = step_x * tens_step as f64;

            for (idx, val) in (start_x as i32..=max_x as i32).step_by(step_x as usize).enumerate() {
                let x = (half_width + 40. * -(idx as f32)) - 40.;
    
                let text = format!("{}", -val as f32 / tens_start as f32);
    
                draw_text(&text, x - text.len() as f32 * (x_half_font / 2.), half_height + x_half_font + 7., x_font_size, BLACK);
                draw_line(x, half_height + 4., x, half_height - 4., 3., DARKGRAY);    
            }
        }

        // y-axis
        draw_line(half_width, 0.0, half_width, screen_height(), COORD_THICKNESS, GRAY);
        
        // x-axis
        draw_line(0.0, half_height, screen_width(), half_height, COORD_THICKNESS, GRAY);


        let mut coords = Vec::new();

        for i in 0..xs.len() {
            let x = xs[i] as f32;
            let y = ys[i] as f32;
    

            let x = half_width + 40. * x;
            let y = half_height - 40. * y;

            coords.push((x, y));

            if marker.contains('o') {
                draw_circle(x, y, 5., GREEN);
            }
            

            if coords.len() >= 2 {
                draw_line(coords[0].0, coords[0].1, coords[1].0, coords[1].1, 3., GREEN);
                coords.remove(0);
            }
            

        }
        next_frame().await;
    }
}

/* 
fn main() {
    let conf = Conf {
        window_width: 325,
        window_height: 325,
        ..Default::default()
    };
    //macroquad::Window::from_config(conf, run(3));
}
*/
