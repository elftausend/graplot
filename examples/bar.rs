use graplot::Bar;

fn main() {
    let mut bar = Bar::new(["Ferris", "Stefan", "Test"], &[100., 200., 700.]);
    bar.set_title("title");
    bar.set_xlabel("test");
    bar.show();
}