use graplot::Plot3D;

#[test]
fn test_3d() {
    let xs = [0.,1.,2.,3.,4.,5.,6.];
    let ys = [0.,1.,4.,9.,16.,25.,36.];
    let zs = [0.,1.,4.,9.,16.,25.,36.];
    
    let plot = Plot3D::new((xs, ys, zs, "r-o"));
    plot.show();
}

#[test]
fn test_sub_zero_3d() {
    let xs = [-1.,1.,2.,3.,4.,5.,6.];
    let ys = [0.,1.,4.,9.,16.,25.,36.];
    let zs = [0.,1.,4.,9.,16.,25.,36.];
    
    let plot = Plot3D::new((xs, ys, zs, "r-o"));
    plot.show();
}