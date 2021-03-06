use litequad::{prelude::{next_frame, clear_background, WHITE, vec3, LIGHTGRAY, BLACK, is_key_pressed, KeyCode, draw_text, vec2, screen_width, screen_height, Vec3, Color, is_key_down}, camera::{set_camera, Camera3D, set_default_camera, Camera}, models::{draw_line_3d, draw_sphere}};

use crate::{max_display, Plot3D, max_matrix, Marker};

pub async fn run(plot: Plot3D) {

    /*let xs = [-1., 1., 2.,];
    let ys = [1., 1., 2.,];
    let zs = [-1., 1., 3.];*/
    let xs = plot.xs;
    let ys = plot.ys;
    let zs = plot.zs;

    let mut max_x = max_matrix(&xs);
    max_x = max_display(max_x, false);
    let steps_x = 6.;
    let step_x = max_x / steps_x;
    
    let mut max_y = max_matrix(&ys);
    max_y = max_display(max_y, false);
    let steps_y = 6.;
    let step_y = max_y / steps_y;

    let mut max_z = max_matrix(&zs);
    max_z = max_display(max_z, false);
    let steps_z = 6.;
    let step_z = max_z / steps_z;

    
    let yaw: f32 = 1.18; //1.18

    let front = vec3(
        yaw.cos(),
        0.,
        yaw.sin(),
    )
    .normalize();

    let world_up = vec3(0.0, 0.0, 1.0);
    let right = front.cross(world_up).normalize();

    let mut position = vec3(-14., 10., 13.);

    loop {
        clear_background(WHITE);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_down(KeyCode::A) {
            //camera_z -= 0.3;
            position -= front;
        }

        if is_key_down(KeyCode::D) {
            //camera_z += 0.3;
            position += front;
        }

        if is_key_down(KeyCode::W) {
            position -= right;
        }

        if is_key_down(KeyCode::S) {
            position += right;
        }

        let camera = Camera3D {
            //position: vec3(camera_x, 10., camera_z),
            position,
            //position: vec3(-14., 12., 10.),
            
            //position: vec3(-10., 7., 11.5),
            

            //position: vec3(-14., 12., -7.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        };

        set_camera(&camera);

        let mut draw_later = Vec::<(f64, Vec3)>::new();

        
        let spacing_z = 1.6;
        let spacing_x = 1.;
        
        let slices_bot = 6;
        let slices_left = 6;
        let slices_back = 6;
        
        draw_bottom_grid(slices_bot, slices_left, spacing_x, spacing_z);
        draw_left_grid(slices_left, spacing_x, spacing_z);     
        draw_back_grid(slices_back, spacing_x, spacing_z);

        let half_slices = (slices_bot as i32) / 2;
            
        // z
        draw_line_3d(
            vec3(half_slices as f32 * spacing_x, 0., half_slices as f32 * spacing_z), 
            vec3(half_slices as f32 * spacing_x, slices_back as f32 * spacing_x, half_slices as f32 * spacing_z),
            BLACK
        );

        // x front
        draw_line_3d(
            vec3(-half_slices as f32 * spacing_x, 0., -half_slices as f32 * spacing_z), 
            vec3(-half_slices as f32 * spacing_x, 0., half_slices as f32 * spacing_z), 
            BLACK
        );

        // x back
        draw_line_3d(
            vec3(half_slices as f32 * spacing_x, 0., -half_slices as f32 * spacing_z), 
            vec3(half_slices as f32 * spacing_x, 0., half_slices as f32 * spacing_z), 
            BLACK
        );


        // z - right
        draw_line_3d(
            vec3(half_slices as f32 * spacing_x, 0., half_slices as f32 * spacing_z), 
            vec3(-half_slices as f32 * spacing_x, 0., half_slices as f32 * spacing_z), 
            BLACK
        );

        // z - left
        draw_line_3d(
            vec3(half_slices as f32 * spacing_x, 0., -half_slices as f32 * spacing_z), 
            vec3(-half_slices as f32 * spacing_x, 0., -half_slices as f32 * spacing_z), 
            BLACK
        );
        
        // lines for z? values
        for (idx, i) in (-half_slices+1..half_slices).into_iter().enumerate() {
            draw_line_3d(
                vec3((-half_slices as f32 + 0.2) * spacing_x, 0., i as f32 * spacing_z), 
                vec3((-half_slices as f32 - 0.2) * spacing_x, 0., i as f32 * spacing_z), 
                BLACK
            );
            draw_later.push(((idx+1) as f64 * step_x, vec3((-half_slices as f32 + 0.4) * spacing_x - 2.1, 0., i as f32 * spacing_z)));
        }

        // lines for x? values
        for (idx, i) in (-half_slices+1..half_slices).into_iter().enumerate() {
            draw_line_3d(
                vec3(i as f32 * spacing_x, 0., (half_slices as f32 - 0.1) * spacing_z), 
                vec3(i as f32 * spacing_x, 0., (half_slices as f32 + 0.1) * spacing_z), 
                BLACK
            );

            draw_later.push(((idx+1) as f64 * step_z, vec3(i as f32 * spacing_x - 0.1, 0., (half_slices as f32 - 0.1) * spacing_z + 0.9)));
        }

        // lines for y? values
        for i in 1..=slices_bot {
            draw_line_3d(
                vec3(half_slices as f32 * spacing_x, i as f32 * spacing_x, (half_slices as f32 - 0.1) * spacing_z), 
                vec3(half_slices as f32 * spacing_x, i as f32 * spacing_x, (half_slices as f32 + 0.1) * spacing_z), 
                BLACK
            );
            draw_later.push((i as f64 * step_y, vec3(half_slices as f32 * spacing_x, i as f32 * spacing_x -0.1, (half_slices as f32 - 0.1) * spacing_z + 0.6)));
        }


        /*
        draw_line_3d(
            vec3(half_slices as f32 * spacing_x, 0., 4.), 
            vec3(half_slices as f32 * spacing_x,0., 6.), 
            litequad::color::GREEN
        );
        */

        /*
        let x = 0.;
        let y = 0.;
        let z = 0.;

        let x1 = 6.;
        let y1 = 4.;
        let z1 = 5.;
        
        let start = vec3((x - half_slices as f32) * spacing_x, y * spacing_x, (z - half_slices as f32 +1.) * spacing_z);
        let end = vec3((x1 - half_slices as f32) * spacing_x, y1 * spacing_x, (z1 - half_slices as f32 + 1.) * spacing_z);

        draw_line_3d(start, end, GREEN);
        */

        
        //let xs = [];
        //let ys = [];
        //let zs = [];
        


        //let max = max_matrix(&vec![x.to_vec(), y.to_vec(), z.to_vec()]);

        let slices = 1;
        for (idx, xs) in xs.iter().enumerate() {
            let mut coords = Vec::new();
            let mut shadow: Vec<(f32, f32, f32)> = Vec::new();
    
            let ys = &ys[idx];
            let zs = &zs[idx];

            let line_desc = &plot.line_desc[idx];

            for i in 0..xs.len() {

                let z = ((xs[i] / step_x) * slices as f64) as f32 /*+ spacing_x*/;
                let y = ((ys[i] / step_y) * slices as f64) as f32;
                //let z = ((zs[i] / 6.) * slices as f64) as f32;
                let x = ((zs[i] / step_z) * slices as f64) as f32;
    
    
                //let transform = camera.matrix().project_point3(vec3(x, y, z));
                //let transform = vec3(x - (half_slices-1) as f32 * spacing_x, y, z - (half_slices-1) as f32 * spacing_z);
                let transform = vec3((x - half_slices as f32) * spacing_x, y * spacing_x, (z - half_slices as f32) * spacing_z);            
                
                
    
                let (x, y, z) = transform.into();
                //println!("({x}, {y}, {z})");
    //            let x = half_width + spacing_x * x;
    //            let y = half_height - spacing_y * y;
    
                match line_desc.marker {
                    Marker::Circle(_) => {
                        draw_sphere(transform, 0.1, None, line_desc.color);
                        draw_sphere((x, 0., z).into(), 0.065, None, Color::new(0., 0., 0., 0.2))
                    },
                    Marker::None => {},
                }
                
                shadow.push((x, 0., z));
                coords.push((x, y, z));
    
                if coords.len() >= 2 {
                    match line_desc.line_type {
                        crate::LineType::Solid => {
                            draw_line_3d(
                                coords[0].into(),
                                coords[1].into(),
                                line_desc.color
                            );
            
                            draw_line_3d(
                                shadow[0].into(),
                                shadow[1].into(),
                                Color::new(0., 0., 0., 0.2)
                            );
                        },
                        crate::LineType::None => {},
                    }
                    
    
                    coords.remove(0);
                    shadow.remove(0);
                }
             
            }
        
        }
        
        //draw_sphere(vec3(0., 0., 0.), 1., None, GREEN);

        let mat = camera.matrix();

        set_default_camera();

        for draw in draw_later {
            let transform = mat.project_point3(vec3(draw.1.x, draw.1.y, draw.1.z));

            let a = vec2(
                (transform.x / 2. + 0.5) * screen_width(),
                (0.5 - transform.y / 2.) * screen_height(),
            );
            let text = format!("{:.3}", draw.0);
            draw_text(&text, a.x, a.y, 15., BLACK);
                
        }

        /* 
        let transform = mat.project_point3(vec3(half_slices as f32 * spacing_x, 0., 4.));

        let a = vec2(
            (transform.x / 2. + 0.5) * screen_width(),
            (0.5 - transform.y / 2.) * screen_height(),
        );
    
        draw_text("-5", a.x, a.y, 15., litequad::color::GREEN);
        */
        
        next_frame().await;
        /*let wait = if pressed {
            16
        } else {
            16
        };*/
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}


pub fn draw_bottom_grid(slices_z: u32, slices_x: u32, spacing_x: f32, spacing_z: f32) {

    let half_slices_x = (slices_x as i32) / 2;
    let half_slices_z = (slices_z as i32) / 2;

    for i in -half_slices_x..=half_slices_x {
        draw_line_3d(
            vec3(i as f32 * spacing_x, 0., -half_slices_z as f32 * spacing_z),
            vec3(i as f32 * spacing_x, 0., half_slices_z as f32 * spacing_z),
            LIGHTGRAY,
        );    
    }

    for i in -half_slices_z..=half_slices_z {
        
        draw_line_3d(
            vec3(-half_slices_x as f32 * spacing_x, 0., i as f32 * spacing_z),
            vec3(half_slices_x as f32 * spacing_x, 0., i as f32 * spacing_z),
            LIGHTGRAY,
        );
    }
}

pub fn draw_left_grid(slices: u32, spacing_x: f32, spacing_z: f32) {
    let half_slices = (slices as i32) / 2;
    for i in -half_slices..=half_slices {
        draw_line_3d(
            vec3(i as f32 * spacing_x, 0., -half_slices as f32 * spacing_z),
            vec3(i as f32 * spacing_x, slices as f32 * spacing_x, -half_slices as f32 * spacing_z),
            LIGHTGRAY,
        );
        draw_line_3d(
            vec3(-half_slices as f32 * spacing_x, (i+half_slices) as f32 * spacing_x, -half_slices as f32 * spacing_z),
            vec3(half_slices as f32 * spacing_x, (i+half_slices) as f32 * spacing_x, -half_slices as f32 * spacing_z),
            LIGHTGRAY,
        );
    }
}

pub fn draw_back_grid(slices: u32, spacing_x: f32, spacing_z: f32) {
    let half_slices = (slices as i32) / 2;
    for i in -half_slices..=half_slices {
        draw_line_3d(
            vec3(half_slices as f32 * spacing_x, (i+half_slices) as f32 * spacing_x, -half_slices as f32 * spacing_z), 
            vec3(half_slices as f32 * spacing_x, (i+half_slices) as f32 * spacing_x, half_slices as f32* spacing_z), 
            LIGHTGRAY
        );
        draw_line_3d(
            vec3(half_slices as f32 * spacing_x, 0., i as f32 * spacing_z), 
            vec3(half_slices as f32 * spacing_x, slices as f32 * spacing_x, i as f32 * spacing_z),
            LIGHTGRAY
        );
    }
}