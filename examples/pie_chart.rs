use graplot::Pie;

fn main() {
    // without labels: let pie = Pie::new([35., 25., 25., 15.]);
    let draw = [(35., "label"), (25., "len"), (25., "labeled"), (15., "test")];
    let pie = Pie::new(draw);
    pie.show();
}