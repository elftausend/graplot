use graplot::solve_lu;

#[test]
fn test_les() {
    let cols = 2;
    let (c, e) = (&[-3., 1., 2., 1.], &[-4., -5.]);

    let res = solve_lu(cols, c, e);

    assert_eq!(
        vec![-0.2, -4.6],
        res.iter().map(|x| *x as f32).collect::<Vec<f32>>()
    );
}
