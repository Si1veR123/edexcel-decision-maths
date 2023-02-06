use std::{cell::RefCell, rc::Rc, fmt::{Display, Debug}};

#[macro_export]
macro_rules! node {
    ($i: literal) => {
        Rc::new(RefCell::new(Node::empty_id($i)))
    };
    ($i: ident) => {
        Rc::new(RefCell::new(Node::empty_id($i)))
    };
    () => {
        Rc::new(RefCell::new(Node::empty()))
    };
}

fn display_id(id: Option<usize>) -> String {
    match id {
        Some(id) => id.to_string(),
        None => String::from("?")
    }
}


#[derive(Debug)]
pub enum EdgeType {
    UndirectedWeighted,
    DirectedWeighted,
    UndirectedUnweighted,
    DirectedUnweighted
}

#[derive(PartialEq)]
pub struct Node {
    // assumes that nodes only have one edge to each other, especially in undirected graphs
    // this is because undirected edges are modelled as 2 directed edges

    // Rc allows multiple nodes to have a reference to each other
    // RefCell provides interior mutability for nodes to be able to mutate each other
    // f64 is the weight of the edge
    edges: Vec<(Rc<RefCell<Node>>, f64)>,
    pub id: Option<usize>
}

impl Node {
    fn details(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node {}\nEdges to: {:?}", display_id(self.id), self.edges.iter().map(|x|{ 
            // include weight if > 0
            match x.1 > 0.0 {
                true => format!("Node {} - Weight: {}", display_id(x.0.borrow().id), x.1),
                false => format!("Node {}", display_id(x.0.borrow().id)),
            }
        }).collect::<Vec<String>>())
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.details(f)
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.details(f)
    }
}

impl Node {
    pub fn empty() -> Self {
        Self { edges: Vec::new(), id: None }
    }
    pub fn empty_id(id: usize) -> Self {
        Self { edges: Vec::new(), id: Some(id) }
    }

    pub fn add_weighted_directed_edge(start: Rc<RefCell<Node>>, end: Rc<RefCell<Node>>, weight: f64) {
        start.borrow_mut().edges.push((end, weight));
    }
    pub fn add_unweighted_directed_edge(start: Rc<RefCell<Node>>, end: Rc<RefCell<Node>>) {
        Self::add_weighted_directed_edge(start, end, 0.0);
    }
    pub fn add_weighted_undirected_edge(start: Rc<RefCell<Node>>, end: Rc<RefCell<Node>>, weight: f64) {
        Self::add_weighted_directed_edge(start.clone(), end.clone(), weight);
        Self::add_weighted_directed_edge(end.clone(), start.clone(), weight);
    }
    pub fn add_unweighted_undirected_edge(start: Rc<RefCell<Node>>, end: Rc<RefCell<Node>>) {
        Self::add_weighted_undirected_edge(start, end, 0.0);
    }

    pub fn out_edges(&self) -> &Vec<(Rc<RefCell<Node>>, f64)> {
        &self.edges
    }

    pub fn edge_type_to(&self, other_node: Rc<RefCell<Node>>) -> Option<EdgeType> {
        // if weighted, edge weight from self -> other_node > 0
        // if directed, there is no returning edge from other node

        let i = self.edges.iter().position(|x| std::ptr::eq(x.0.as_ptr(), other_node.as_ptr()));
        if i.is_none() { return None }

        let edge = self.edges.get(i.unwrap()).unwrap();
        let is_weighted = edge.1 > 0.0;

        let is_directed = {
            // if the edge is directed, then the other node doesn't have a returning edge
            let j = edge.0.borrow().out_edges().iter().position(|x| std::ptr::eq(x.0.as_ptr(), self));
            j.is_none()
        };

        match (is_directed, is_weighted) {
            (true, true) => Some(EdgeType::DirectedWeighted),
            (true, false) => Some(EdgeType::DirectedUnweighted),
            (false, true) => Some(EdgeType::UndirectedWeighted),
            (false, false) => Some(EdgeType::UndirectedUnweighted)
        }
    }
}
