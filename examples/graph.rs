use graplot::{Graph, RED, graph::GraphDesc, Color};

fn main() {
    let mut graph = Graph::new();
    graph.graph_desc = GraphDesc {
        node_color: RED,
        outer_ring: (Color::new(1., 0.5, 0.8, 1.), 3.5),
        ..Default::default()
    };
    let a = graph.add_node(vec![]);
    let b = graph.add_node(vec![]);
    let c = graph.add_node(vec![]);
    
    let d = graph.add_node(vec![a.idx, b.idx]);
    let e = graph.add_node(vec![a.idx, c.idx]);

    let _f = graph.add_node(vec![d.idx, e.idx, b.idx]);

    graph.show();
}