use crate::{consts::Consts, GenericFraction};

pub trait Trigonometrics {
    fn atan(&self) -> Self;
    fn atan2(&self, x: &Self) -> Self;
}

impl<T: Clone + num::Integer + From<u64>> Trigonometrics for GenericFraction<T>
where GenericFraction<T>: Consts {
    fn atan(&self) -> Self {
        let x = self;
        &Self::FRAC_PI_4 * x - x * (x.abs() - Self::new(1_u64, 1_u64)) * (Self::new(2447_u64, 10000_u64) + Self::new(663_u64, 10000_u64) * x.abs())
    }
    fn atan2(&self, x: &Self) -> Self {
        let y = self;
        let zero = Self::new(0_u64, 0_u64);
        match(x >= &zero, y >= &zero, y < x) {
            (true, true, true) => {
                (x / y).atan()
            },
            (true, true, false) => {
                Self::FRAC_PI_2 - (x /y).atan()
            },
            (true, false, true) => {
                (y / x).atan()
            },
            (true, false, false) => {
                -Self::FRAC_PI_2 - (x /y).atan()
            },
            (false, true, true) => {
                (y / x).atan() + Self::PI
            },
            (false, true, false) => {
                Self::FRAC_PI_2 - (x / y).atan()
            },
            (false, false, true) => {
                (y / x).atan() - Self::PI
            },
            (false, false, false) => {
                -Self::FRAC_PI_2 - (x / y).atan()
            },
        }
    }
}
