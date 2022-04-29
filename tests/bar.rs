use graplot::Bar;


#[test]
fn test_basic_bar() {
    let bar = Bar::new(["Ferris", "Stefan"], &[100., 200.,]);
    println!("{:?}", bar.bars);
}