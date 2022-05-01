use graplot::Pie;

#[test]
fn test_pie() {
    let pie = Pie::new([35., 25., 25., 15.]);
    pie.show();
}

#[test]
fn test_pie_labeled() {
    let pie = Pie::new([(25., "label"), (10., "10!"), (25., "len"), (5., "lowest"), (25., "labeled"), (10., "test")]);
    pie.show();
}