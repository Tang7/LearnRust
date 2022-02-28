use std::rc::Rc;

#[derive(Debug)]
struct Node {
    _id: usize,
    leaf: Option<Rc<Node>>,
}

impl Node {
    pub fn new(_id: usize) -> Self {
        Self { _id, leaf: None }
    }

    pub fn update_leaf(&mut self, leaf: Rc<Node>) {
        self.leaf = Some(leaf)
    }

    pub fn get_leaf(&self) -> Option<Rc<Node>> {
        self.leaf.as_ref().map(|v| v.clone())
    }
}

fn main() {
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);
    let node4 = Node::new(4);

    node3.update_leaf(Rc::new(node4));
    node1.update_leaf(Rc::new(node3));
    node2.update_leaf(node1.get_leaf().unwrap());

    println!("node1: {:?}, node2: {:?}", node1, node2);
}
