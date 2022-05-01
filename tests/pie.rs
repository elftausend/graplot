use graplot::Pie;

#[test]
fn test_pie() {
    let pie = Pie::new([35., 25., 25., 15.]);
    pie.show();
}