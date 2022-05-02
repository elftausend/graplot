use graplot::{Bar, RED, GREEN, BLUE, ORANGE};

#[test]
fn test_basic_bar() {
    let mut bar = Bar::new([("Ferris", RED), ("Stefan", GREEN), ("Test", BLUE)], &[100., 200., 500.,]);
    bar.add(("Added", ORANGE), 400.);
    bar.set_title("title");
    bar.set_ylabel("test");
    bar.show();
}