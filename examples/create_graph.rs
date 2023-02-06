extern crate decision_maths;
use std::{cell::RefCell, rc::Rc};

use decision_maths::{node, graphs::Node};

fn main() {
    let first_node = node!(1);
    let second_node = node!(2);
    let third_node = node!(3);
    Node::add_unweighted_undirected_edge(first_node.clone(), second_node.clone());
    Node::add_weighted_directed_edge(third_node.clone(), first_node.clone(), 5.0);
    println!("{}", first_node.borrow());
    println!("{}", second_node.borrow());
    println!("{}", third_node.borrow());

    println!("{:?}", third_node.borrow().edge_type_to(first_node.clone()));
}
