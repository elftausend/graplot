use graplot::Pie;

#[test]
fn test_pie() {
    let pie = Pie::new([35., 25., 25., 15.]);
    pie.show();
}

#[test]
fn test_pie_labeled() {
    let draw = [(35., "label"), (25., "len"), (25., "labeled"), (15., "test")];
    let mut pie = Pie::new(draw);
    pie.set_title("title");
    pie.show();
}