

async fn run() {
    loop {
        clear_background(WHITE);
        next_frame().await;
    }
}