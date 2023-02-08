use std::{rc::{Rc, Weak}, cell::{RefCell, Ref}};
use crate::{algorithm::SteppedAlgorithm, graphs::Node};

// node, value, final?, weak ref to node which provided the working value
type DijkstrasNode = (Rc<RefCell<Node>>, f64, bool, Option<Weak<RefCell<Node>>>);

pub struct Dijkstras {
    nodes: Vec<RefCell<DijkstrasNode>>,
    last_updated: usize, // index into nodes vec
    end: Weak<RefCell<Node>>
}

impl Dijkstras {
    pub fn new(start_node: Rc<RefCell<Node>>, end_node: Weak<RefCell<Node>>) -> Self {
        let start_node_dijkstras = (start_node, 0.0, true, None);
        let nodes = vec![RefCell::new(start_node_dijkstras)];

        Self { nodes, last_updated: 0, end: end_node }
    }

    fn get_dijkstras_node_i(&self, node: &Rc<RefCell<Node>>) -> Option<usize> {
        self.nodes.iter().position(|x| x.borrow().0.as_ptr() == node.as_ptr())
    }

    fn update_working_values(&mut self) {
        let mut new_node_buffer = vec![];

        // iterate over all outgoing nodes from last_updated that aren't finished
        {
            // FIRST BORROW FROM self.nodes IS IMMUTABLE AND ON THE LAST NODE
            let last_node_dijkstras: Ref<DijkstrasNode> = self.nodes.get(self.last_updated).unwrap().borrow();
            let actual_node_ref = last_node_dijkstras.0.borrow();

            let next_nodes = actual_node_ref.out_edges();

            for node in next_nodes.iter() {
                // SECOND BORROW FROM self.nodes IS IMMUTABLE (borrow's lifetime doesn't live past this method)
                let i = self.get_dijkstras_node_i(&node.0);
                        
                let node_to_update = match i {
                    Some(i) => self.nodes.get(i).unwrap(),
                    None => {
                        let new_node: DijkstrasNode = (node.0.clone(), f64::INFINITY, false, None);
                        new_node_buffer.push(RefCell::new(new_node));
                        new_node_buffer.last().unwrap()
                    }
                };

                // THIRD BORROW FROM self.nodes IS MUTABLE
                let mut node_to_update_mut = node_to_update.borrow_mut();

                if node_to_update_mut.2 {
                    // node is finished
                    continue;
                }

                let new_possible_working_value = node.1 + last_node_dijkstras.1;
                if new_possible_working_value < node_to_update_mut.1 {
                    node_to_update_mut.3 = Some(Rc::downgrade(&last_node_dijkstras.0));
                    node_to_update_mut.1 = new_possible_working_value;
                }
            }
        }

        self.nodes.extend(new_node_buffer.into_iter());
    }

    fn least_working_value(&self) -> usize {
        // returns index in self.nodes of the dijkstras node that isn't finished and has the lowest working value

        // index, value
        let mut min: (usize, f64) = (usize::MAX, f64::INFINITY);

        for (i, node) in self.nodes.iter()
            .enumerate() {

            if node.borrow().2 {
                continue;
            }

            let node_ref = node.borrow();
            if node_ref.1 < min.1 {
                min = (i, node_ref.1);
            }
        }

        min.0
    }

    fn backtrack_from_last(&self) -> (Vec<Rc<RefCell<Node>>>, f64) {
        let n = self.end.upgrade().unwrap();
        let current_node_i = self.get_dijkstras_node_i(&n);

        if current_node_i.is_none() {
            return (vec![], 0.0);
        }

        let mut current_node = self.nodes.get(current_node_i.unwrap()).unwrap();

        let init_data = {
            let b = current_node.borrow();
            (b.0.clone(), b.1)
        };

        let mut node_path = vec![init_data.0];

        loop {
            let previous_node_ref_opt = &current_node.borrow().3;
            if previous_node_ref_opt.is_none() {
                return (node_path, init_data.1);
            }

            let previous_node_weak = previous_node_ref_opt.as_ref().unwrap();
            let previous_node = previous_node_weak.upgrade().unwrap();
            let previous_node_i = self.get_dijkstras_node_i(&previous_node).unwrap();

            let previous_node_dijkstras = self.nodes.get(previous_node_i).unwrap();
            let n_ref = previous_node_dijkstras.borrow();

            node_path.push(n_ref.0.clone());

            current_node = previous_node_dijkstras;
        }
    }
}

impl SteppedAlgorithm for Dijkstras {
    type ReturnType = (Vec<Rc<RefCell<Node>>>, f64);

    fn step(&mut self) -> bool {
        self.update_working_values();
        let to_finalise = self.least_working_value();
        self.last_updated = to_finalise;

        let mut finalise_node = self.nodes.get(to_finalise).unwrap().borrow_mut();

        let end_node = self.end.upgrade().unwrap();

        finalise_node.2 = true;

        if std::ptr::eq(end_node.as_ptr(), finalise_node.0.as_ptr()) {
            return true;
        }
        false
    }

    fn run(mut self) -> Self::ReturnType {
        while !self.step() {};
        self.backtrack_from_last()
    }
}
