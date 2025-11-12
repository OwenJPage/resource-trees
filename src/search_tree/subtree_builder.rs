use {
    crate::search_tree::new_node::{
        NewNode,
        UTrait,
    },
    std::{
        cell::RefCell,
        mem::MaybeUninit,
    },
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

pub trait SubtreeBuilder<R> {
    type Child;
    fn add_child(&self) -> Self::Child;

    fn to_subtree(self) -> R;
}

pub struct SubtreeBuilderRoot<'node, U: UTrait> {
    parent:   &'node mut NewNode<U>,
    children: RefCell<Vec<SubtreeBuilderChild<'node, U>>>,
}

impl<'node, U: UTrait> SubtreeBuilder<NewNode<U>> for SubtreeBuilderRoot<'node, U> {
    type Child = &'node SubtreeBuilderChild<'node, U>;

    fn add_child(&self) -> Self::Child {
        todo!("Create child node and add to vec, return reference to child")
    }

    fn to_subtree(self) -> NewNode<U> {
        // Convert children to nodes, add nodes to root, add to parent

        todo!()
    }
}

impl<U: UTrait> Drop for SubtreeBuilderRoot<'_, U> {
    fn drop(&mut self) {
        let subtree = unsafe { std::ptr::read(self) }.to_subtree();
    }
}

pub struct SubtreeBuilderChild<'root, U: UTrait> {
    root:     &'root SubtreeBuilderRoot<'root, U>,
    children: Vec<&'root Self>,
}

impl<'root, U: UTrait> SubtreeBuilder<NewNode<U>> for SubtreeBuilderChild<'root, U> {
    type Child = &'root Self;

    fn add_child(&self) -> Self::Child {
        todo!("Create child node and add to parent's vec")
    }

    fn to_subtree(self) -> NewNode<U> {
        todo!()
    }
}
