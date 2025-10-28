use {
    crate::{
        dose::Dose,
        node::Node,
        pill_count::PillCount,
    },
    std::{
        collections::VecDeque,
        rc::Rc,
    },
};

#[cfg(test)]
mod test;

pub struct SearchTree {
    roots:  Box<[Rc<Node>]>,
    tops:   Box<[Rc<Node>]>,
    height: usize,
    size:   usize,
}

impl SearchTree {
    pub fn build<D: IntoIterator<Item = Dose>>(doses: D, starting_stock: PillCount) -> Self {
        let mut roots = Vec::new();
        let mut tops = Vec::new();
        let mut height: usize = 0;
        let mut size: usize = 0;

        let mut current = vec![Node::new_start_node(starting_stock)];

        'doses: for dose in doses {
            let pill_combinations = PillCount::find_combinations(dose.amount)
                .expect("Total dose amount is not divisible by available pill sizes.");

            for _dose_day in 0..dose.days {
                let mut next: Vec<Rc<Node>> = Vec::new(); // TODO: Use clever capacity

                for node in &current {
                    for pill_count in pill_combinations.as_ref() {
                        if let Some(child_node) = node.add_child(*pill_count) {
                            next.push(child_node);
                            size += 1;
                        }
                    }
                }

                // TODO: Optimise, currently branching with every iteration
                if roots.is_empty() {
                    next.clone_into(&mut roots);
                }

                if next.is_empty() {
                    break 'doses;
                } else {
                    current = next;
                    height += 1;
                }
            }
        }

        current.clone_into(&mut tops);

        Self {
            roots: roots.into_boxed_slice(),
            tops: tops.into_boxed_slice(),
            height,
            size,
        }
    }

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
