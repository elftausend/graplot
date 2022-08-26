use std::collections::HashMap;

use litequad::{window::{next_frame, clear_background, screen_width}, prelude::WHITE, shapes::{draw_circle, draw_line}};
use crate::{Graph, graph::EdgeColor};
use super::COLOR_ARRAY;

pub async fn run(graph: Graph) {
 
    let radius = graph.graph_desc.node_radius;
    let node_distance_x = graph.graph_desc.node_distance_x;
    let node_distance_y = graph.graph_desc.node_distance_y;
    let layers = graph.layers();

    let mut node_coords = HashMap::<usize, (f32, f32)>::new();
    
    loop {
        clear_background(WHITE);

        let mut node_count = 0;
        for layer in &layers {
            let length = (radius * 2. + (node_distance_x - radius *2.)) * layer.nodes.len() as f32 - (node_distance_x - radius*2.);
            
            
            for (idx, node) in layer.nodes.iter().enumerate() {
                let x = screen_width() / 2. + node_distance_x * idx as f32 - length / 2.;
                let y = 30. + node_distance_y * layer.layer as f32;
                node_coords.insert(node.idx, (x, y));

                draw_circle(x, y, radius, graph.graph_desc.node_color);

                //draw_line(screen_width() / 2. - radius, y, screen_width() / 2. - radius +length, y, 5., BLUE);

                for dep in &node.deps {
                    if let Some((prev_x, prev_y)) = node_coords.get(dep) {
                        
                        let color = match graph.graph_desc.egde_color {
                            EdgeColor::Use(color) => color,
                            EdgeColor::Mixed => COLOR_ARRAY[node_count % COLOR_ARRAY.len()],
                        };
                        draw_line(*prev_x, prev_y + radius, x, y - radius, 2., color);
                    }
                    
                }
                node_count += 1;
            }
            
        }
        next_frame().await;
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
    
}