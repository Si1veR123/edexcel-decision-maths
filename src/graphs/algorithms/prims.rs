
use crate::algorithm::SteppedAlgorithm;
use crate::graphs::Node;
use std::{cell::RefCell, rc::Rc};

type MappingNode = (Rc<RefCell<Node>>, Rc<RefCell<Node>>);

fn create_empty_node_from_node(node: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
    // creates an empty node, with the same id as the passed node if an id exists, else id is None
    let id = node.borrow().id;
    let new_graph_node = Rc::new(RefCell::new(
        match id {
            Some(id) => Node::empty_id(id),
            None => Node::empty(),
        }
    ));

    new_graph_node
}

pub struct Prims {
    // a vector of visited nodes, and the node which represents the node in the new MST graph
    // this new graph ONLY HAS unweighted undirected edges

    visited: Vec<MappingNode>,
    target_nodes: usize,
}

impl Prims {
    pub fn new(start_node: Rc<RefCell<Node>>, nodes_count: usize) -> Self {
        let new_graph_start_node = create_empty_node_from_node(start_node.clone());
        Self { visited: vec![(start_node, new_graph_start_node)], target_nodes: nodes_count}
    }
    pub fn run(mut self) -> Vec<Rc<RefCell<Node>>> {
        while !self.step() {};
        self.visited.into_iter().map(|(_, x)| x).collect()
    }
}

impl SteppedAlgorithm for Prims {
    fn step(&mut self) -> bool {
        if self.visited.len() == self.target_nodes {
            return true
        }

        // ( (from original, from representation), to, weight)
        let mut minimum_edge: (Option<MappingNode>, Option<Rc<RefCell<Node>>>, f64) = (None, None, f64::INFINITY);
        for node in self.visited.iter() {
            for edge in node.0.borrow().out_edges() {
                if 
                    (edge.1 < minimum_edge.2) &
                    self.visited.iter().position(|(x, _)| std::ptr::eq(x.as_ptr(), edge.0.as_ptr())).is_none()
                {
                    minimum_edge = (Some(node.clone()), Some(edge.0.clone()), edge.1);
                }
            }
        }

        // no more nodes (reached MST)
        if minimum_edge.0.is_none() {
            return true
        }

        let original_node_to = minimum_edge.1.unwrap();

        let new_representation_node_to = create_empty_node_from_node(original_node_to.clone());
        let representation_node_from = minimum_edge.0.unwrap().1;

        // add edge on new graph
        Node::add_unweighted_undirected_edge(new_representation_node_to.clone(), representation_node_from);

        // add the original node and new representation node to visited
        self.visited.push((original_node_to, new_representation_node_to));
        
        false
    }
}
