use graplot::Graph;


#[test]
fn test_graph() {
    let mut graph = Graph::new();
    let a = graph.add_node(vec![]);
    let b = graph.add_node(vec![]);
    let e = graph.add_node(vec![]);

    let c = graph.add_node(vec![a.idx, b.idx]);
    //let d = graph.add_node(vec![a.idx, b.idx]);
    let d = graph.add_node(vec![e.idx, b.idx]);
    let f = graph.add_node(vec![a.idx, d.idx]);
    
    let g = graph.add_node(vec![f.idx, d.idx, c.idx]);

    
    graph.show();
}