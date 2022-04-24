use graplot::{x, Desc, Plot, XEnd};

#[test]
fn test_desc() {
    let desc = Desc {
        end: x(2.),
        ..Default::default()
    };
    assert_eq!(
        Desc {
            end: XEnd(2.),
            spacing_x: 40.,
            spacing_y: 40.,
            min_steps_x: 4.,
            min_steps_y: 4.
        },
        desc
    );
}

#[test]
fn test_desc_render() {
    let mut plot = Plot::new([1., 2., 3., 4., 5.]);
    plot.set_desc(Desc {
        spacing_x: 180.,
        ..Default::default()
    });
    plot.show();
}
