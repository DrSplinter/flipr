use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;

use crate::Offset;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Place {
    pub(super) x: Decimal,
    pub(super) y: Decimal,
}

impl Place {
    pub fn new(x: f64, y: f64) -> Option<Self> {
        let x = Decimal::from_f64(x)?;
        let y = Decimal::from_f64(y)?;

        Some(Self { x, y })
    }

    pub const ORIGIN: Self = Self {
        x: Decimal::ZERO,
        y: Decimal::ZERO,
    };

    pub const fn origin() -> Self {
        Self::ORIGIN
    }

    pub fn offset_to(self, other: Self) -> Offset {
        other - self
    }
}

impl std::ops::Add<Offset> for Place {
    type Output = Place;

    fn add(self, rhs: Offset) -> Self::Output {
        let x = self.x + rhs.dx;
        let y = self.y + rhs.dy;

        Self { x, y }
    }
}

impl std::ops::AddAssign for Place {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Sub for Place {
    type Output = Offset;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;

        Offset { dx: x, dy: y }
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::Strategy;
    use proptest::proptest;

    use super::*;

    fn finite_f64() -> impl Strategy<Value = f64> {
        proptest::num::f64::NEGATIVE | proptest::num::f64::POSITIVE | proptest::num::f64::ZERO
    }

    fn offset_gen() -> impl Strategy<Value = Offset> {
        (finite_f64(), finite_f64()).prop_map(|(x, y)| Offset::new(x, y).expect("finite f64s"))
    }

    fn place_gen() -> impl Strategy<Value = Place> {
        (finite_f64(), finite_f64()).prop_map(|(x, y)| Place::new(x, y).expect("finite f64s"))
    }

    proptest! {
        #[test]
        fn offset_zero_place_add_right_identity(p in place_gen()) {
            assert_eq!(p + Offset::ZERO, p)
        }

        #[test]
        fn place_add_offset_add_associative(p in place_gen(), a in offset_gen(), b in offset_gen()) {
            assert_eq!((p + a) + b, p + (a + b))
        }

        #[test]
        fn place_add_place_sub(p in place_gen(), q in place_gen()) {
            assert_eq!(p + (q - p), q)
        }
    }
}
