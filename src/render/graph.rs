use std::collections::HashMap;

use litequad::{window::{next_frame, clear_background, screen_width}, prelude::{WHITE, GREEN, BLUE}, shapes::{draw_circle, draw_circle_lines, draw_line, draw_rectangle}};

use crate::Graph;

use super::{COLOR_ARRAY, DISTANCE_X_AXIS};

const LAYER_DISTANCE_Y: f32 = 65.;
const LAYER_DISTANCE_X: f32 = 55.;
const RADIUS: f32 = 20.;

pub async fn run(graph: Graph) {
 
    let layers = graph.layers();

    let mut node_coords = HashMap::<usize, (f32, f32)>::new();
    
    loop {
        clear_background(WHITE);

//        draw_rectangle(screen_width() / 2. - 100., 50., 200., 150., GREEN);

        //draw_circle_lines(30., 30., 20.,  3., GREEN);
        
        

        let mut node_count = 0;
        for layer in &layers {
            let length = (RADIUS * 2. + (LAYER_DISTANCE_X - RADIUS *2.)) * layer.nodes.len() as f32 - (LAYER_DISTANCE_X - RADIUS*2.);
            
            
            for (idx, node) in layer.nodes.iter().enumerate() {
                let x = screen_width() / 2. + LAYER_DISTANCE_X * idx as f32 - length / 2.;
                let y = 30. + LAYER_DISTANCE_Y * layer.layer as f32;
                node_coords.insert(node.idx, (x, y));

                draw_circle(x, y, RADIUS, GREEN);

                //draw_line(screen_width() / 2. - RADIUS, y, screen_width() / 2. - RADIUS +length, y, 5., BLUE);

                for dep in &node.deps {
                    if let Some((prev_x, prev_y)) = node_coords.get(dep) {
                        draw_line(*prev_x, prev_y + RADIUS, x, y - RADIUS, 2., COLOR_ARRAY[node_count % COLOR_ARRAY.len()]);
                    }
                    
                }
                node_count += 1;
            }
            
        }
        
        next_frame().await;
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
    
}