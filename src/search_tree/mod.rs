use {
    self::subtree_index::SubtreeIndex,
    crate::{
        HeightType,
        SizeType,
        dose::Dose,
        pill_count::PillCount,
    },
    node::Node,
    std::{
        collections::VecDeque,
        rc::Rc,
    },
};

mod new_node;
mod node;
mod subtree_builder;
mod subtree_index;
#[cfg(test)]
mod test;

pub struct SearchTree {
    root: Rc<Node>,
    tops: Box<[Rc<Node>]>,
}

impl SearchTree {
    pub fn build<D: IntoIterator<Item = Dose>>(doses: D, starting_stock: PillCount) -> Self {
        let root = Node::new_start_node(starting_stock);
        let mut tops = Vec::new();

        let mut current = vec![root.clone()];

        'doses: for dose in doses {
            let pill_combinations = PillCount::find_combinations(dose.amount)
                .expect("Total dose amount is not divisible by available pill sizes.");

            for _dose_day in 0..dose.days {
                let mut next: Vec<Rc<Node>> =
                    Vec::with_capacity(current.len() * pill_combinations.len());

                for node in &current {
                    let mut children_adder = node.add_children();
                    for pill_count in pill_combinations.as_ref() {
                        if let Some(child_node) = children_adder.add_child(*pill_count) {
                            next.push(child_node);
                        }
                    }
                }

                if next.is_empty() {
                    break 'doses;
                } else {
                    current = next;
                }
            }
        }

        current.clone_into(&mut tops);

        Self {
            root,
            tops: tops.into_boxed_slice(),
        }
    }

    fn new_subtree(root: Rc<Node>) -> Self {
        Self {
            root,
            tops: Vec::new().into_boxed_slice(),
        }
    }

    #[inline]
    pub fn height(&self) -> HeightType {
        self.root.get_height()
    }

    #[inline]
    pub fn size(&self) -> SizeType {
        self.root.get_size()
    }

    // pub fn subtree<I: SubtreeIndex>(&self, child_index: I) -> Option<Self> {
    //     let mut cur = self.root.clone();
    //
    //     for index in child_index.get_index() {
    //         let child = cur.children.borrow().get(index)?.clone();
    //
    //         cur = child;
    //     }
    //
    //     cur
    // }

    pub fn best_routes(&self) -> Box<[Box<[PillCount]>]> {
        self.tops.iter().map(|t| {
            let mut current = t.clone();

            let mut doses = VecDeque::from([t.get_dose()]);

            while let Some(current) = current.get_parent() {
                doses.push_front(current.get_dose());
            }
        });

        todo!()
    }
}
