extern crate decision_maths;
use std::{cell::RefCell, rc::Rc};

use decision_maths::{node, graphs::{Node, algorithms::Dijkstras}, algorithm::SteppedAlgorithm};

fn main() {
    let mut nodes = Vec::new();
    for i in 0..6 {
        nodes.push(node!(i));
    }

    /*
           B ======= 15 ========== F
        10/    \9             /8   |
       /            \     /        |
      C ==== 12 ====== A           | 12
        \8          /9    \11      |
          \     /              \   |
            D ======== 14 ======== E

    A: 0,
    B: 1,
    ...

    */

    // add bidirectional weighted arcs
    Node::add_weighted_undirected_edge(nodes[0].clone(), nodes[1].clone(), 9.0);
    Node::add_weighted_undirected_edge(nodes[0].clone(), nodes[2].clone(), 12.0);
    Node::add_weighted_undirected_edge(nodes[0].clone(), nodes[3].clone(), 9.0);
    Node::add_weighted_undirected_edge(nodes[0].clone(), nodes[4].clone(), 11.0);
    Node::add_weighted_undirected_edge(nodes[0].clone(), nodes[5].clone(), 8.0);
    Node::add_weighted_undirected_edge(nodes[1].clone(), nodes[5].clone(), 15.0);
    Node::add_weighted_undirected_edge(nodes[1].clone(), nodes[2].clone(), 10.0);
    Node::add_weighted_undirected_edge(nodes[2].clone(), nodes[3].clone(), 8.0);
    Node::add_weighted_undirected_edge(nodes[3].clone(), nodes[4].clone(), 14.0);
    Node::add_weighted_undirected_edge(nodes[4].clone(), nodes[5].clone(), 12.0);

    let prims_algo = Dijkstras::new(nodes.get(2).unwrap().clone(), Rc::downgrade(nodes.get(5).unwrap()));
    let prims_result = prims_algo.run();

    println!("{:?}", prims_result);
}
