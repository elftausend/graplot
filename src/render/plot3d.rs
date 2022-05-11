use litequad::{prelude::{next_frame, clear_background, WHITE, vec3, LIGHTGRAY, BLACK, is_key_pressed, KeyCode, draw_text, vec2, screen_width, screen_height, Vec3, GREEN, Color}, camera::{set_camera, Camera3D, set_default_camera, Camera}, models::{draw_line_3d, draw_sphere}};

pub async fn run() {
    loop {
        clear_background(WHITE);

        let camera = Camera3D {
            //position: vec3(-17., 12., 0.),
            //position: vec3(-14., 12., 10.),
            position: vec3(-10., 9., 11.5),
            //position: vec3(-14., 12., -7.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        };

        set_camera(&camera);

        let mut draw_later = Vec::<Vec3>::new();

        let slices = 6;
        let spacing_z = 1.6;
        let spacing_x = 1.;
        
        draw_grid(slices, spacing_x, spacing_z);
        
        let half_slices = (slices as i32) / 2;
        
        for i in -half_slices..=half_slices {
            let color = LIGHTGRAY;

            draw_line_3d(
                vec3(i as f32 * spacing_x, 0., -half_slices as f32 * spacing_z),
                vec3(i as f32 * spacing_x, slices as f32 * spacing_x, -half_slices as f32 * spacing_z),
                color,
            );
            draw_line_3d(
                vec3(-half_slices as f32 * spacing_x, (i+half_slices) as f32 * spacing_x, -half_slices as f32 * spacing_z),
                vec3(half_slices as f32 * spacing_x, (i+half_slices) as f32 * spacing_x, -half_slices as f32 * spacing_z),
                color,
            );
        }
        
        // back grid
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

        // z (up)
        draw_line_3d(
            vec3(half_slices as f32 * spacing_x, 0., half_slices as f32 * spacing_z), 
            vec3(half_slices as f32 * spacing_x, slices as f32 * spacing_x, half_slices as f32 * spacing_z),
            BLACK
        );

        draw_line_3d(
            vec3(-half_slices as f32 * spacing_x, 0., -half_slices as f32 * spacing_z), 
            vec3(-half_slices as f32 * spacing_x, 0., half_slices as f32 * spacing_z), 
            BLACK
        );


        draw_line_3d(
            vec3(half_slices as f32 * spacing_x, 0., half_slices as f32 * spacing_z), 
            vec3(-half_slices as f32 * spacing_x, 0., half_slices as f32 * spacing_z), 
            BLACK
        );
        
        // lines for y? values
        for i in -half_slices+1..half_slices {
            draw_line_3d(
                vec3((-half_slices as f32 + 0.2) * spacing_x, 0., i as f32 * spacing_z), 
                vec3((-half_slices as f32 - 0.2) * spacing_x, 0., i as f32 * spacing_z), 
                BLACK
            );
            draw_later.push(vec3((-half_slices as f32 + 0.2) * spacing_x - 1.2, 0., i as f32 * spacing_z));
        }

        // lines for x? values
        for i in -half_slices+1..half_slices {
            draw_line_3d(
                vec3(i as f32 * spacing_x, 0., (half_slices as f32 - 0.1) * spacing_z), 
                vec3(i as f32 * spacing_x, 0., (half_slices as f32 + 0.1) * spacing_z), 
                BLACK
            );

            draw_later.push(vec3(i as f32 * spacing_x - 0.1, 0., (half_slices as f32 - 0.1) * spacing_z + 0.5));
        }

        // lines for z? values
        for i in 1..=slices {
            draw_line_3d(
                vec3(half_slices as f32 * spacing_x, i as f32 * spacing_x, (half_slices as f32 - 0.1) * spacing_z), 
                vec3(half_slices as f32 * spacing_x, i as f32 * spacing_x, (half_slices as f32 + 0.1) * spacing_z), 
                BLACK
            );
            draw_later.push(vec3(half_slices as f32 * spacing_x, i as f32 * spacing_x -0.1, (half_slices as f32 - 0.1) * spacing_z + 0.4));
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
        let xs = [0.,1.,2.,3.,4.,5.,6.];
        let ys = [0.,1.,4.,9.,16.,25.,36.];
        let zs = [0.,1.,4.,9.,16.,25.,36.];
        
        let mut coords = Vec::new();

        let mut shadow: Vec<(f32, f32, f32)> = Vec::new();


        //let max = max_matrix(&vec![x.to_vec(), y.to_vec(), z.to_vec()]);

        let slices = 1;

        for i in 0..xs.len() {

            let z = ((xs[i] / 1.) * slices as f64) as f32;
            let y = ((ys[i] / 6.) * slices as f64) as f32;
            //let z = ((zs[i] / 6.) * slices as f64) as f32;
            let x = ((zs[i] / 6.) * slices as f64) as f32;


            //let transform = camera.matrix().project_point3(vec3(x, y, z));
            //let transform = vec3(x - (half_slices-1) as f32 * spacing_x, y, z - (half_slices-1) as f32 * spacing_z);
            let transform = vec3((x - half_slices as f32) * spacing_x, y * spacing_x, (z - half_slices as f32 ) * spacing_z);            
            draw_sphere(transform, 0.1, None, GREEN);
            

            let (x, y, z) = transform.into();
            //println!("({x}, {y}, {z})");
//            let x = half_width + spacing_x * x;
//            let y = half_height - spacing_y * y;

            draw_sphere((x, 0., z).into(), 0.065, None, Color::new(0., 0., 0., 0.2));

            shadow.push((x, 0., z));
            coords.push((x, y, z));

            if coords.len() >= 2 {
                draw_line_3d(
                    coords[0].into(),
                    coords[1].into(),
                    GREEN
                );

                draw_line_3d(
                    shadow[0].into(),
                    shadow[1].into(),
                    Color::new(0., 0., 0., 0.2)
                );

                coords.remove(0);
                shadow.remove(0);
            }
         
        }
    
        //draw_sphere(vec3(0., 0., 0.), 1., None, GREEN);

        let mat = camera.matrix();

        set_default_camera();

        for draw in draw_later {
            let transform = mat.project_point3(vec3(draw.x, draw.y, draw.z));

            let a = vec2(
                (transform.x / 2. + 0.5) * screen_width(),
                (0.5 - transform.y / 2.) * screen_height(),
            );
            
            draw_text("-5", a.x, a.y, 15., litequad::color::GREEN);
                
        }

        /* 
        let transform = mat.project_point3(vec3(half_slices as f32 * spacing_x, 0., 4.));

        let a = vec2(
            (transform.x / 2. + 0.5) * screen_width(),
            (0.5 - transform.y / 2.) * screen_height(),
        );
    
        draw_text("-5", a.x, a.y, 15., litequad::color::GREEN);
        */
        
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        
        next_frame().await;
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}


pub fn draw_grid(slices: u32, spacing_x: f32, spacing_z: f32) {
    let half_slices = (slices as i32) / 2;
    for i in -half_slices..=half_slices {
        //let color = if i == 0 { BLACK } else { LIGHTGRAY };
        let color = LIGHTGRAY;

        draw_line_3d(
            vec3(i as f32 * spacing_x, 0., -half_slices as f32 * spacing_z),
            vec3(i as f32 * spacing_x, 0., half_slices as f32 * spacing_z),
            color,
        );    
        draw_line_3d(
            vec3(-half_slices as f32 * spacing_x, 0., i as f32 * spacing_z),
            vec3(half_slices as f32 * spacing_x, 0., i as f32 * spacing_z),
            LIGHTGRAY,
        );
    }
}