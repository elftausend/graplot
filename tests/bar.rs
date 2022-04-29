use graplot::Bar;


#[test]
fn test_basic_bar() {
    let bar = Bar::new(["Ferris", "Stefan", "Test"], &[100., 200., 700.]);
    println!("{:?}", bar.bars);
    bar.show();
}