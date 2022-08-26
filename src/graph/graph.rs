use litequad::window::Conf;
use crate::render;

#[derive(Debug, Default)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub title: String,
}

impl Graph {
    pub fn new() -> Graph {
        Graph::default()
    }
    pub fn add_node(&mut self, deps: Vec<usize>) -> Node {
        let idx = self.nodes.len();

        let node = Node {
            idx,
            deps
        };
        self.nodes.push(node.clone());
        node
    }

    pub fn layers(&self) -> Vec<Layer> {
        let mut layers = vec![Layer::new(0)];


        for node in &self.nodes {
            if node.deps.is_empty() {
                layers[0].add_node(node)
            }
        }
        
        while count_nodes_in_layers(&layers) < self.nodes.len() {
            let mut next_layer = Layer::new(layers.len());
            for node in &self.nodes {
                if node.deps.is_empty() {
                    continue;
                }
    
                for layer_node in &layers[layers.len() -1].nodes {
                    if node.deps.contains(&layer_node.idx) && !next_layer.nodes_contains_deps(&node.deps) {
                        next_layer.add_node(node);
                        break;
                    }
                }
            }
            layers.push(next_layer);
        }
        layers
    }

    pub fn show(self) {
        
        let conf = Conf {
            window_title: self.title.clone(),
            window_width: 395,
            window_height: 395,
            ..Default::default()
        };
        litequad::Window::from_config(conf, render::graph::run(self));
    }
}

fn count_nodes_in_layers(layers: &[Layer]) -> usize {
    let mut node_count = 0;
    for layer in layers {
        node_count += layer.nodes.len();
    }
    node_count
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Layer<'a> {
    pub layer: usize,
    pub nodes: Vec<&'a Node>,
}

impl<'a> Layer<'a> {
    pub fn new(layer: usize) -> Self {
        Layer {
            layer,
            nodes: vec![]
        }
    }
    pub fn add_node(&mut self, node: &'a Node) {
        self.nodes.push(node)
    }

    pub fn nodes_contains_deps(&self, deps: &[usize]) -> bool {
        for node in &self.nodes {
            if deps.contains(&node.idx) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Node {
    pub idx: usize,
    pub deps: Vec<usize>,
}

