use litequad::{prelude::{next_frame, clear_background, WHITE, vec3, LIGHTGRAY}, camera::{set_camera, Camera3D}, models::draw_line_3d};


pub async fn run() {
    loop {
        clear_background(WHITE);

        set_camera(&Camera3D {
            position: vec3(-20., 15., 0.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        draw_grid(6, 1., 1.6);

        next_frame().await;
    }
}


pub fn draw_grid(slices: u32, spacing_x: f32, spacing_z: f32) {
    let half_slices = (slices as i32) / 2;
    for i in -half_slices..half_slices + 1 {
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
            color,
        );
    }
}