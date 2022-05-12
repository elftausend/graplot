use graplot::Pie;
use litequad::prelude::Color;

fn main() {
    // without labels: let pie = Pie::new([35., 25., 25., 15.]);
    let draw = [(35., "label"), (25., "len"), (25., "labeled"), (15., "test")];
    let mut pie = Pie::new(draw);
    pie.color(0, Color::new(0.6, 0.2, 0.3, 1.));
    pie.show();
}