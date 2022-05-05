use litequad::{prelude::{next_frame, clear_background, WHITE, vec3, LIGHTGRAY, BLACK, draw_text}, camera::{set_camera, Camera3D}, models::draw_line_3d};


pub async fn run() {
    loop {
        clear_background(WHITE);

        set_camera(&Camera3D {
            //position: vec3(-30., 10., 0.),
            //position: vec3(-17., 12., 0.),
            position: vec3(-14., 12., 10.),
            //position: vec3(-5., 15., 5.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

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
        }

        // lines for x? values
        for i in -half_slices+1..half_slices {
            draw_line_3d(
                vec3(i as f32 * spacing_x, 0., (half_slices as f32 - 0.1) * spacing_z), 
                vec3(i as f32 * spacing_x, 0., (half_slices as f32 + 0.1) * spacing_z), 
                BLACK
            );
        }

        // lines for z? values
        for i in 1..=slices {
            draw_line_3d(
                vec3(half_slices as f32 * spacing_x, i as f32 * spacing_x, (half_slices as f32 - 0.1) * spacing_z), 
                vec3(half_slices as f32 * spacing_x, i as f32 * spacing_x, (half_slices as f32 + 0.1) * spacing_z), 
                BLACK
            );
        }

        draw_text("test", 0., 0., 10., BLACK);
        
        next_frame().await;
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