use graplot::{Desc, Plot};

fn main() {
    let mut list: Vec<f64> = Vec::new();
    let input: f64 = 1000.;
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
        let a: f64 = list[list.len() - 2];
        let b: f64 = list[list.len() - 1];
        println!("Input {} ended in a loop between {} and {}", input, a, b);
        println!("Highest Point: {}", highest_point);
        println!("Lowest Point: {}", lowest_point);

        let mut plot = Plot::new(list);
        plot.set_desc(Desc {
            min_steps_x: 10.,
            spacing_x: 47.,
            ..Default::default()
        });
        plot.set_color((0.1, 0.6, 0.9));

        plot.show();
    }
}
