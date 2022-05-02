use graplot::{Bar, ORANGE, RED, GREEN, BLUE};

fn main() {
    let mut bar = Bar::new([("Ferris", RED), ("Stefan", GREEN), ("Test", BLUE)], &[100., 200., 500.,]);
    bar.add(("Added", ORANGE), 400.);
    bar.set_title("title");
    bar.set_ylabel("test");
    bar.show();
}