use {
    crate::search_tree::new_node::{
        NewNode,
        UTrait,
    },
    std::cell::RefCell,
};

impl SubtreeBuilderParent for () {}

pub trait SubtreeBuilderParent {}

pub trait SubtreeBuilderSelfParent {}
// impl<P: SubtreeBuilderSelfParent> SubtreeBuilderParent for P {}
//
// impl<P: SubtreeBuilderParent> SubtreeBuilderSelfParent for SubtreeBuilder<'_,
// P> {}
//
// pub struct SubtreeBuilder<'node, P: SubtreeBuilderParent> {
//     parent:   &'node mut dyn SubtreeBuilderParent,
//     children: Vec<SubtreeBuilder<'node, Self>>,
// }
//
// impl<'node, U: UTrait> SubtreeBuilder<'node, NewNode<U>> {
//     pub fn new(parent: &'node mut NewNode<U>) -> Self {
//         Self {
//             parent,
//             children: Vec::new(),
//         }
//     }
// }
//
// pub struct VecSubtreeBuilderChild<'root, U: UTrait> {
//     root: &'root mut VecSubtreeBuilderRoot<'root, U>,
// }
//
// pub struct VecSubtreeBuilderRoot<'this, U: UTrait> {
//     parent:   Option<NewNode<U>>,
//     children: VecDeque<VecSubtreeBuilderChild<'this, U>>,
// }

pub trait SubtreeBuilder {
    type Child;
    fn add_child() -> Self::Child;
}

pub struct SubtreeBuilderRoot<'node, U: UTrait> {
    parent: &'node mut NewNode<U>,
    children: RefCell<Vec<SubtreeBuilderChild<'node, U>>>,
}

impl<'node, U: UTrait> SubtreeBuilder for SubtreeBuilderRoot<'node, U> {
    type Child = &'node SubtreeBuilderChild<'node, U>;

    fn add_child() -> Self::Child {
        todo!("Create child node and add to vec, return reference to child")
    }
}

pub struct SubtreeBuilderChild<'root, U: UTrait> {
    root: &'root SubtreeBuilderRoot<'root, U>,
    children: Vec<&'root Self>,
}

impl<'root, U: UTrait> SubtreeBuilder for SubtreeBuilderChild<'root, U> {
    type Child = &'root Self;

    fn add_child() -> Self::Child {
        todo!("Create child node and add to parent's vec")
    }
}
