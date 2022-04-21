use graplot::{Desc, Plot, PlotArg};

fn main() {
    let mut plot = Plot::default();
    plot.set_title("Collatz Conjecture");
    plot.set_desc(Desc {
        min_steps_x: 10.,
        spacing_x: 47.,
        ..Default::default()
    });

    for input in 1000..=1001 {
        let mut single_graph = collatz(input as f64).as_plot();
        single_graph.set_color(0., 1. * (input == 1000) as i32 as f32, 1.);
        plot.add(single_graph);
    }
    plot.show();
}

fn collatz(input: f64) -> Vec<f64> {
    let mut list: Vec<f64> = Vec::new();
    if input != 0.0 {
        let mut step: f64 = input;
        let mut highest_point: f64 = input;
        let mut lowest_point: f64 = input;

        while !list.contains(&step) {
            list.push(step);
            if step % 2.0 == 0.0 {
                step /= 2.0;
            } else {
                step = step * 3.0 + 1.0
            }
            if step > highest_point {
                highest_point = step;
            }
            if step < lowest_point {
                lowest_point = step;
            }
        }
        list.push(step);
    }
    list
}
