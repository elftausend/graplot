use graplot::{Desc, Plot, x};

fn main() {
    let mut plot = Plot::new((|x: f64| x.cos(), x(5.)));
    plot.set_desc(Desc {
        min_steps_x: 6.,
        spacing_x: 47.,
        ..Default::default()
    });
    plot.show();
}
