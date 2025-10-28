use crate::{
    AmountType,
    CountType,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PillCount {
    pub ten_mg:    CountType,
    pub twenty_mg: CountType,
}

impl PillCount {
    #[inline]
    pub(crate) const fn new(ten_mg: CountType, twenty_mg: CountType) -> Self {
        Self { ten_mg, twenty_mg }
    }

    pub fn find_combinations(total_dose: AmountType) -> Option<Box<[Self]>> {
        // TODO: Not have pill types hardcoded

        if !total_dose.is_multiple_of(10) {
            return None;
        }

        let mut combinations = Vec::with_capacity((total_dose / 20 + 1) as usize);

        for (twenty_count, twenty_dose) in (0..=total_dose).step_by(20).enumerate() {
            combinations.push(Self::new(
                ((total_dose - twenty_dose) / 10) as CountType,
                twenty_count as CountType,
            ));
        }

        Some(combinations.into_boxed_slice())
    }

    pub fn total(&self) -> u16 {
        self.ten_mg as u16 + self.twenty_mg as u16
    }

    pub fn checked_sub(&self, rhs: Self) -> Option<Self> {
        Some(Self::new(
            self.ten_mg.checked_sub(rhs.ten_mg)?,
            self.twenty_mg.checked_sub(rhs.twenty_mg)?,
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_eq() {
        let a = PillCount {
            ten_mg:    3,
            twenty_mg: 7,
        };

        let b = PillCount {
            ten_mg:    3,
            twenty_mg: 7,
        };

        assert_eq!(a, b)
    }

    #[test]
    fn test_ne() {
        let a = PillCount::new(3, 7);
        let b = PillCount::new(5, 7);

        assert_ne!(a, b)
    }

    mod find_combinations {
        use super::*;

        #[test]
        #[should_panic = "`find_combinations` did not complete successfully"]
        fn test_5_none() {
            PillCount::find_combinations(5)
                .expect("`find_combinations` did not complete successfully");
        }

        #[test]
        fn test_10_some() {
            let combinations = PillCount::find_combinations(10)
                .expect("`find_combinations` did not complete successfully");

            println!("Combinations: {:#?}", combinations);

            let expected_combinations = vec![PillCount::new(1, 0)].into_boxed_slice();

            assert_eq!(combinations, expected_combinations);
        }

        #[test]
        fn test_60_some() {
            let combinations = PillCount::find_combinations(60)
                .expect("`find_combinations` did not complete successfully");

            println!("Combinations: {:#?}", combinations);

            let expected_combinations = vec![
                PillCount::new(6, 0),
                PillCount::new(4, 1),
                PillCount::new(2, 2),
                PillCount::new(0, 3),
            ]
            .into_boxed_slice();

            assert_eq!(combinations, expected_combinations);
        }

        #[test]
        #[should_panic = "`find_combinations` did not complete successfully"]
        fn test_75_none() {
            PillCount::find_combinations(75)
                .expect("`find_combinations` did not complete successfully");
        }
    }

    mod checked_sub {
        use super::*;

        #[test]
        fn test_some() {
            let a = PillCount::new(5, 7);
            let b = PillCount::new(3, 7);

            let c = a
                .checked_sub(b)
                .expect("`checked_sub` did not complete successfully");

            let expected_c = PillCount::new(2, 0);

            assert_eq!(c, expected_c);
        }

        #[test]
        #[should_panic = "`checked_sub` did not complete successfully"]
        fn test_none() {
            let a = PillCount::new(5, 5);
            let b = PillCount::new(3, 7);

            let c = a
                .checked_sub(b)
                .expect("`checked_sub` did not complete successfully");
        }
    }
}
