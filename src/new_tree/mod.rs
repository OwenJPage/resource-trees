use std::{
    cell::RefCell,
    collections::HashMap,
    hash::Hash,
    rc::Rc,
};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct NodeID<R: Copy + Eq + Hash> {
    level:           usize,
    remaining_stock: R,
}

pub struct TreeNodeInner<R: Copy + Eq + Hash> {
    id:       NodeID<R>,
    children: Vec<NodeID<R>>,
}

#[derive(Clone)]
pub struct TreeNode<R: Copy + Eq + Hash> {
    inner: Rc<RefCell<TreeNodeInner<R>>>,
}

pub struct NewTree<R: Copy + Eq + Hash> {
    nodes: HashMap<NodeID<R>, TreeNode<R>>,
}

impl<R: Copy + Eq + Hash> NewTree<R> {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }
}
