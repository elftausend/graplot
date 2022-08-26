use graplot::Graph;


#[test]
fn test_graph() {
    let mut graph = Graph::default();
    let a = graph.add_node(vec![]);
    let b = graph.add_node(vec![]);

    let c = graph.add_node(vec![a.idx, b.idx]);
    //let d = graph.add_node(vec![a.idx, b.idx]);
    let e = graph.add_node(vec![c.idx, b.idx]);

    graph.show();

}