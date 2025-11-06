use {
    crate::{
        DayType,
        HeightType,
        SizeType,
        pill_count::PillCount,
    },
    std::{
        cell::RefCell,
        marker::PhantomData,
        rc::{
            Rc,
            Weak,
        },
    },
};

pub struct ChildrenAdder<'node> {
    phantom:      PhantomData<&'node ()>,
    node:         Rc<Node>,
    new_children: Vec<Rc<Node>>,
}

impl<'node> ChildrenAdder<'node> {
    #[inline]
    fn new(node: &Rc<Node>) -> Self {
        Self {
            phantom:      PhantomData::default(),
            node:         node.clone(),
            new_children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, dose_pills: PillCount) -> Option<Rc<Node>> {
        let child = self.node.new_child(dose_pills)?;

        self.new_children.push(child.clone());

        Some(child)
    }

    pub fn new_add_child(&mut self, dose_pills: PillCount) -> Option<Self> {
        let child = self.node.new_child(dose_pills)?;

        self.new_children.push(child.clone());

        Some(Self::new(&child))
    }
}

impl From<ChildrenAdder<'_>> for Rc<Node> {
    #[inline]
    fn from(value: ChildrenAdder<'_>) -> Self {
        value.node.clone()
    }
}

impl Drop for ChildrenAdder<'_> {
    #[inline]
    fn drop(&mut self) {
        let mut children = self.node.children.borrow_mut();

        if children.is_empty() {
            self.node.add_to_height(1);
        }

        self.node.add_to_size(self.new_children.len() as SizeType);
        children.append(&mut self.new_children);
    }
}

pub trait Parent {}

impl Parent for Rc<Node> {}

pub trait ParentAdder {}

impl<P: ParentAdder> Parent for P {}

pub struct RecChildrenAdder<'node, P: Parent> {
    // this:   Weak<Self>,
    parent: &'node mut P,
}

impl<'node, P: Parent> ParentAdder for RecChildrenAdder<'node, P> {}

impl<'node, P: Parent> RecChildrenAdder<'node, P> {
    #[inline]
    pub fn new(parent: &'node mut P) -> Self {
        Self { parent }
    }

    pub fn add_child(
        &'node mut self,
        dose_pills: PillCount,
    ) -> Option<RecChildrenAdder<'node, Self>> {
        let child = RecChildrenAdder::new(self);

        Some(child)
    }
}

pub struct Node {
    this: Weak<Self>,
    day: DayType,
    parent: Weak<Self>,
    pub(in crate::search_tree) children: RefCell<Vec<Rc<Self>>>,

    /// The total number of nodes for this node's subtree, _**including**_ this
    /// node.
    size:   RefCell<SizeType>,
    /// The number of layers of nodes _**above**_ this node.
    height: RefCell<HeightType>,

    pills_taken: PillCount,
    pill_stock:  PillCount,
}

impl Node {
    #[inline]
    fn new(
        parent: &Weak<Self>,
        day: DayType,
        pills_taken: PillCount,
        pill_stock: PillCount,
    ) -> Rc<Self> {
        Rc::new_cyclic(|this| {
            Self {
                this: this.clone(),
                parent: parent.clone(),
                children: Default::default(),
                size: 1.into(),
                height: 0.into(),
                day,
                pills_taken,
                pill_stock,
            }
        })
    }

    #[inline]
    pub fn get_height(&self) -> HeightType {
        *self.height.borrow()
    }

    #[inline]
    pub fn get_size(&self) -> SizeType {
        *self.size.borrow()
    }

    #[inline]
    fn add_to_height(&self, amount: HeightType) {
        *self.height.borrow_mut() += amount;

        if let Some(parent) = self.parent.upgrade() {
            parent.add_to_height(amount);
        }
    }

    #[inline]
    fn add_to_size(&self, amount: SizeType) {
        *self.size.borrow_mut() += amount;

        if let Some(parent) = self.parent.upgrade() {
            parent.add_to_size(amount);
        }
    }

    #[inline]
    pub fn new_start_node(starting_pill_stock: PillCount) -> Rc<Self> {
        Self::new(&Weak::new(), 0, PillCount::new(0, 0), starting_pill_stock)
    }

    #[inline]
    fn new_child(&self, dose_pills: PillCount) -> Option<Rc<Self>> {
        Some(Self::new(
            &self.this,
            self.day + 1,
            dose_pills,
            self.pill_stock.checked_sub(dose_pills)?,
        ))
    }

    #[inline]
    pub fn add_children(&self) -> ChildrenAdder {
        ChildrenAdder::new(
            &self
                .this
                .upgrade()
                .expect("Node `this` weak reference is invalid"),
        )
    }

    #[inline]
    pub fn get_parent(&self) -> Option<Rc<Self>> {
        self.parent.upgrade()
    }

    #[inline]
    pub fn get_dose(&self) -> PillCount {
        self.pills_taken
    }
}
