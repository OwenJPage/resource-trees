use {
    crate::search_tree::new_node::{
        NewNode,
        UTrait,
    },
    std::collections::VecDeque,
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
