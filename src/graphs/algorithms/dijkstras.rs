use std::{rc::{Rc, Weak}, cell::RefCell};
use crate::{algorithm::SteppedAlgorithm, graphs::Node};

// node, value, final?
type DijkstrasNode = (Rc<RefCell<Node>>, f64, bool);

pub struct Dijkstras {
    nodes: Vec<RefCell<DijkstrasNode>>,
    last_updated: usize, // index into nodes vec
    end: Weak<RefCell<Node>>
}

impl Dijkstras {
    pub fn new(start_node: Rc<RefCell<Node>>, end_node: Weak<RefCell<Node>>) -> Self {
        let start_node_dijkstras = (start_node, 0.0, true);
        let nodes = vec![RefCell::new(start_node_dijkstras)];

        Self { nodes, last_updated: 0, end: end_node }
    }

    fn get_dijkstras_node_i(&self, node: &Rc<RefCell<Node>>) -> Option<usize> {
        self.nodes.iter().position(|x| x.borrow().0.as_ptr() == node.as_ptr())
    }

    fn node_is_finished(&self, node: Rc<RefCell<Node>>) -> bool {
        // return an iterator over finished dijkstras nodes
        let mut filter = self.nodes.iter().filter(|&x| x.borrow().2);
        filter.position(|y|
                std::ptr::eq(y.borrow().0.as_ptr(), node.as_ptr())
        ).is_some()
    }

    fn update_working_values(&mut self) {
        let mut new_node_buffer = vec![];

        // FIRST BORROW FROM self.nodes IS IMMUTABLE AND ON THE LAST NODE
        // iterate over all outgoing nodes from last_updated that aren't finished
        let last_node = self.nodes.get(self.last_updated).unwrap().borrow();

        for node in last_node.0.borrow().out_edges()
            .iter()
            .filter(|&x|
                // SECOND BORROWS FROM self.nodes ARE IMMUTABLE
                self.node_is_finished(x.0.clone())
        ) {
            // THIRD BORROW FROM self.nodes IS IMMUTABLE (borrow's lifetime doesn't live past this method)
            let i = self.get_dijkstras_node_i(&node.0);
                    
            let node_to_update = match i {
                Some(i) => self.nodes.get(i).unwrap(),
                None => {
                    let new_node = (node.0.clone(), f64::INFINITY, false);
                    new_node_buffer.push(RefCell::new(new_node));
                    new_node_buffer.last().unwrap()
                }
            };

            // FOURTH BORROW FROM self.nodes IS MUTABLE
            let mut node_to_update_mut = node_to_update.borrow_mut();

            let new_possible_working_value = node.1 + last_node.1;
            if new_possible_working_value < node_to_update_mut.1 {
                node_to_update_mut.1 = new_possible_working_value;
            }
        }
    }
}

impl SteppedAlgorithm for Dijkstras {
    type ReturnType = Vec<Rc<RefCell<Node>>>;

    fn step(&mut self) -> bool {
        false
    }

    fn run(mut self) -> Self::ReturnType {
        while !self.step() {};
        vec![]
    }
}

