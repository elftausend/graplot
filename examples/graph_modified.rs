use graplot::{Graph, graph::{EdgeColor, GraphDesc}, RED, MAGENTA};

fn main() {
    let mut graph = Graph::new();
    graph.graph_desc = GraphDesc {
        title: String::from("Graph"),
        node_color: RED,
        egde_color: EdgeColor::Use(MAGENTA),
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