use crate::{
    AmountType,
    DayType,
};

#[derive(Copy, Clone)]
pub struct Dose {
    pub days:   DayType,
    pub amount: AmountType,
}

impl Dose {
    pub const fn new(days: DayType, amount: AmountType) -> Self {
        Self { days, amount }
    }
}
