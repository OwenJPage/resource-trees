use {
    crate::{
        DayType,
        HeightType,
        SizeType,
        pill_count::PillCount,
    },
    std::{
        cell::RefCell,
        ops::Sub,
        rc::{
            Rc,
            Weak,
        },
    },
};

trait UTrait: Clone + Copy + Default + Sub {
    fn checked_sub(self, rhs: Self) -> Option<Self>;
}

struct Inner<U: UTrait> {
    parent:          Weak<RefCell<Self>>,
    level:           usize,
    expended:        U,
    remaining_stock: U,
    children:        Vec<NewNode<U>>,
    size:            SizeType,
    height:          HeightType,
}

#[derive(Clone)]
pub struct NewNode<U: UTrait> {
    inner: Rc<RefCell<Inner<U>>>,
}

impl<U: UTrait> NewNode<U> {
    fn new(
        parent: &Weak<RefCell<Inner<U>>>,
        level: usize,
        expended: U,
        remaining_stock: U,
    ) -> Self {
        Self {
            inner: Rc::new(RefCell::new(Inner {
                parent: parent.clone(),
                level: 0,
                expended,
                remaining_stock,
                children: Vec::new(),
                size: 1,
                height: 0,
            })),
        }
    }

    pub fn new_start_node(remaining_stock: U) -> Self {
        Self::new(&Weak::new(), 0, Default::default(), remaining_stock)
    }

    fn new_child(&self, expended: U) -> Option<Self> {
        // Borrowing inner to get the level might cause issues here.
        Some(Self::new(
            &Rc::downgrade(&self.inner),
            self.inner.borrow().level + 1,
            expended,
            self.inner.borrow().remaining_stock.checked_sub(expended)?,
        ))
    }

    pub fn add_child(&self) -> Self {
        let child = Self {
            inner: Rc::new(RefCell::new(Inner {
                parent:          Rc::downgrade(&self.inner),
                level:           self.inner.borrow().level + 1,
                expended:        PillCount::new(0, 0),
                remaining_stock: self.inner.borrow().remaining_stock,
                children:        Vec::new(),
                size:            1,
                height:          0,
            })),
        };

        self.inner.borrow_mut().children.push(child.clone());

        child
    }
}
