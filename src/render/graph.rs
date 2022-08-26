use litequad::{window::{next_frame, clear_background, screen_width}, prelude::{WHITE, GREEN}, shapes::{draw_circle, draw_circle_lines}};

use crate::Graph;


pub async fn run(graph: Graph) {
 
    loop {
        clear_background(WHITE);

        //draw_circle_lines(30., 30., 20.,  3., GREEN);

        
        draw_circle(screen_width() / 2., 30., 20., GREEN);

        next_frame().await;
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
    
}