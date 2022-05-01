use graplot::Bar;


#[test]
fn test_basic_bar() {
    let mut bar = Bar::new(["Ferris", "Stefan", "Test",], &[100., 200., 500.,]);
    println!("{:?}", bar.bars);
    bar.set_title("title");
    bar.set_ylabel("test");
    bar.show();
}