use litequad::window::Conf;
use crate::render;

#[derive(Debug, Default)]
pub struct Graph {
    nodes: Vec<Node>,
    title: String,
}

impl Graph {
    pub fn add_node(&mut self, mut deps: Vec<usize>) -> Node {
        let idx = self.nodes.len();

        let node = Node {
            idx,
            deps
        };
        self.nodes.push(node.clone());
        node
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

#[derive(Debug, Clone)]
pub struct Node {
    pub idx: usize,
    pub deps: Vec<usize>,
}

