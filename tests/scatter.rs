use graplot::{Scatter, Desc};


#[test]
fn test_scatter() {
    let x = [5.,7.,8.,7.,2.,17.,2.,9.,4.,11.,12.,9.,6.];
    let y = [99.,86.,87.,88.,111.,86.,103.,87.,94.,78.,77.,85.,86.];

    let mut scat = Scatter::new((x, y));
    
    scat.plot.set_desc(Desc {
        spacing_x: 100.,
        spacing_y: 100.,
        ..Default::default()
    });
    scat.show();
}