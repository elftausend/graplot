use litequad::{window::Conf, prelude::{Color, GREEN}};
use crate::render;

pub trait CommonGraph {
    fn as_common_graph(&self) -> Graph;
}

/// EgdeColor::Mixed, the color is selected via an array of colors.
#[derive(Debug)]
pub enum EdgeColor {
    Use(Color),
    Mixed,
}

#[derive(Debug)]
pub struct GraphDesc {
    pub title: String,
    pub node_color: Color,
    pub node_radius: f32,
    pub node_distance_x: f32,
    pub node_distance_y: f32,
    pub egde_color: EdgeColor,
    pub window_width: i32,
    pub window_height: i32,

}

impl Default for GraphDesc {
    fn default() -> Self {
        Self { 
            node_color: GREEN, 
            title: Default::default(),
            node_radius: 20.,
            node_distance_x: 55.,
            node_distance_y: 65.,
            egde_color: EdgeColor::Mixed,
            window_width: 395,
            window_height: 395,
        }
    }
}

#[derive(Debug, Default)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub graph_desc: GraphDesc,
}

impl From<Vec<Node>> for Graph {
    fn from(nodes: Vec<Node>) -> Self {
        Graph { nodes, graph_desc: Default::default() }
    }
}

impl Graph {
    pub fn new() -> Graph {
        Graph::default()
    }

    pub fn set_graph_desc(&mut self, graph_desc: GraphDesc) {
        self.graph_desc = graph_desc;
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

    pub fn set_node_color(&mut self, node_color: Color) {
        self.graph_desc.node_color = node_color;
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
            window_title: self.graph_desc.title.clone(),
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

