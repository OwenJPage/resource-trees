use {
    crate::{
        pill_count::PillCount,
        DayType,
    },
    std::{
        cell::RefCell,
        rc::{
            Rc,
            Weak,
        },
    },
};

pub struct Node {
    this:     Weak<Self>,
    day:      DayType,
    parent:   Weak<Self>,
    children: RefCell<Vec<Rc<Self>>>,

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
                children: RefCell::default(),
                day,
                pills_taken,
                pill_stock,
            }
        })
    }

    pub fn new_start_node(starting_pill_stock: PillCount) -> Rc<Self> {
        Self::new(&Weak::new(), 0, PillCount::new(0, 0), starting_pill_stock)
    }

    pub fn add_child(&self, dose_pills: PillCount) -> Option<Rc<Self>> {
        let child = Self::new(
            &self.this,
            self.day + 1,
            dose_pills,
            self.pill_stock.checked_sub(dose_pills)?,
        );

        self.children.borrow_mut().push(child.clone());

        Some(child)
    }

    pub fn get_parent(&self) -> Option<Rc<Self>> {
        self.parent.upgrade()
    }

    pub fn get_dose(&self) -> PillCount {
        self.pills_taken
    }
}
