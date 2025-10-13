use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;

use crate::Scale;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Offset {
    pub(super) dx: Decimal,
    pub(super) dy: Decimal,
}

impl Offset {
    pub const ZERO: Self = Self {
        dx: Decimal::ZERO,
        dy: Decimal::ZERO,
    };

    pub const fn zero() -> Self {
        Self::ZERO
    }

    pub fn new(x: f64, y: f64) -> Option<Self> {
        let dx = Decimal::from_f64(x)?;
        let dy = Decimal::from_f64(y)?;

        Some(Self { dx, dy })
    }
}

impl std::ops::Add for Offset {
    type Output = Offset;

    fn add(self, rhs: Self) -> Self::Output {
        let dx = self.dx + rhs.dx;
        let dy = self.dy + rhs.dy;

        Self { dx, dy }
    }
}

impl std::ops::Neg for Offset {
    type Output = Offset;

    fn neg(self) -> Self::Output {
        let dx = -self.dx;
        let dy = -self.dy;

        Self { dx, dy }
    }
}

impl std::ops::Mul<Scale> for Offset {
    type Output = Offset;

    fn mul(self, rhs: Scale) -> Self::Output {
        let dx = self.dx * rhs.0;
        let dy = self.dy * rhs.0;

        Self { dx, dy }
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

    fn scale_gen() -> impl Strategy<Value = Scale> {
        finite_f64().prop_map(|f| Scale::new(f).expect("finite f64"))
    }

    fn offset_gen() -> impl Strategy<Value = Offset> {
        (finite_f64(), finite_f64()).prop_map(|(x, y)| Offset::new(x, y).expect("finite f64s"))
    }

    proptest! {
        #[test]
        fn offset_add_associative(a in offset_gen(), b in offset_gen(), c in offset_gen()) {
            assert_eq!(a + (b + c), (a + b) + c)
        }

        #[test]
        fn offset_add_commutative(a in offset_gen(), b in offset_gen()) {
            assert_eq!(a + b, b + a)
        }

        #[test]
        fn offset_zero_offset_add_left_identity(a in offset_gen()) {
            assert_eq!(Offset::ZERO + a, a)
        }

        #[test]
        fn offset_zero_offset_add_right_identity(a in offset_gen()) {
            assert_eq!(a + Offset::ZERO, a)
        }

        #[test]
        fn offset_add_inverse(a in offset_gen()) {
            assert_eq!(a + -a, Offset::ZERO)
        }

        #[test]
        fn offset_mul_scale_mul_associative(a in offset_gen(), m in scale_gen(), n in scale_gen()) {
            assert_eq!((a * m) * n, a * (m * n))
        }

        #[test]
        fn scale_one_offset_mul_right_identity(a in offset_gen()) {
            assert_eq!(a * Scale::ONE, a)
        }

        #[test]
        fn offset_mul_distributive_over_offset_add(a in offset_gen(), b in offset_gen(), m in scale_gen()) {
            assert_eq!((a + b) * m, a * m + b * m)
        }

        #[test]
        fn offset_mul_distributive_over_scale_add(a in offset_gen(), m in scale_gen(), n in scale_gen()) {
            assert_eq!(a * (m + n), a * m + a * n)
        }
    }
}
