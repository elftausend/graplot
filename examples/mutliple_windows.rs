use graplot::Plot;

fn main() {
    if cfg!(target_os="linux") {
        let mut plot = Plot::new(|x: f64| x.powf(3.) + x.powf(2.) - 0.08);
        plot.set_title("x^3 + x^2 - 0.08");
        let h = plot.show_threaded();
        
        let mut plot = Plot::new(|x: f64| x.powf(2.) + 0.08);
        plot.set_title("x²");
        plot.show();

        h.join().unwrap() // you need to close both windows
    } else {
        println!("no support")
    }
}

