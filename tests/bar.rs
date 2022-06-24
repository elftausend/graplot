use graplot::{Bar, RED, GREEN, BLUE, ORANGE};

#[test]
fn test_basic_bar() {
    let mut bar = Bar::new([("Ferris", RED), ("Stefan", GREEN), ("Test", BLUE)], &[100., 200., 500.,]);
    bar.add(("Added", ORANGE), 400.);
    bar.set_title("title");
    bar.set_ylabel("test");
    bar.show();
}
#[test]
fn test_neg_basic_bar() {
    let mut bar = Bar::new([("Ferris", RED), ("Stefan", GREEN), ("Test", BLUE)], &[100., 200., 500.,]);
    bar.add(("Added", ORANGE), -400.);
    bar.set_title("title");
    bar.set_ylabel("test");
    bar.show();
}

#[test]
fn test_small_bar() {
    let mut bar = Bar::new([("Ferris", RED), ("Stefan", GREEN), ("Test", BLUE)], &[0.1, 0.2, 0.5,]);
    bar.add(("Added", ORANGE), -0.4);
    bar.set_title("title");
    bar.set_ylabel("test");
    bar.show();
}

#[test]
fn test_only_neg_bar() {
    let mut bar = Bar::new([("Ferris", RED), ("Stefan", GREEN), ("Test", BLUE)], &[-100., -200., -500.,]);
    bar.add(("Added", ORANGE), -400.);
    bar.set_title("title");
    bar.set_ylabel("test");
    bar.show();
}