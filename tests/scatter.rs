use graplot::Scatter;

#[test]
fn test_scatter() {
    let x = [5.,7.,8.,7.,2.,17.,2.,9.,4.,11.,12.,9.,6.];
    let y = [99.,86.,87.,88.,111.,86.,70.,87.,94.,78.,77.,85.,86.];

    let mut scat = Scatter::new((x, y));
    
    scat.plot.set_desc(graplot::Desc {
        spacing_y: 80.,
        ..Default::default()
    });
    
    scat.add(([2., 10.5], [78., 114.], "r-"));
    scat.show();
}