use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Scale(pub(super) Decimal);

impl Scale {
    pub fn new(v: f64) -> Option<Self> {
        Decimal::from_f64(v).map(Self)
    }

    pub const ONE: Self = Self(Decimal::ONE);
    pub const ZERO: Self = Self(Decimal::ZERO);

    pub const fn one() -> Self {
        Self::ONE
    }

    pub const fn zero() -> Self {
        Self::ZERO
    }
}

impl std::ops::Mul for Scale {
    type Output = Scale;

    fn mul(self, rhs: Scale) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl std::ops::Add for Scale {
    type Output = Scale;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Neg for Scale {
    type Output = Scale;

    fn neg(self) -> Self::Output {
        Self(-self.0)
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

    proptest! {
        #[test]
        fn scale_add_associative(a in scale_gen(), b in scale_gen(), c in scale_gen()) {
            assert_eq!(a + (b + c), (a + b) + c)
        }

        #[test]
        fn scale_add_commutative(a in scale_gen(), b in scale_gen()) {
            assert_eq!(a + b, b + a)
        }

        #[test]
        fn scale_zero_scale_add_left_identity(a in scale_gen()) {
            assert_eq!(Scale::ZERO + a, a)
        }

        #[test]
        fn scale_zero_scale_add_right_identity(a in scale_gen()) {
            assert_eq!(a + Scale::ZERO, a)
        }

        #[test]
        fn scale_add_inverse(a in scale_gen()) {
            assert_eq!(a + (-a), Scale::ZERO)
        }

        #[test]
        fn scale_mul_associative(a in scale_gen(), b in scale_gen(), c in scale_gen()) {
            assert_eq!(a * (b * c), (a * b) * c)
        }

        #[test]
        fn scale_mul_commutative(a in scale_gen(), b in scale_gen()) {
            assert_eq!(a * b, b * a)
        }

        #[test]
        fn scale_zero_scale_mul_left_anihilator(a in scale_gen()) {
            assert_eq!(Scale::ZERO * a, Scale::ZERO)
        }

        #[test]
        fn scale_zero_scale_mul_right_anihilator(a in scale_gen()) {
            assert_eq!(a * Scale::ZERO, Scale::ZERO)
        }

        #[test]
        fn scale_one_scale_mul_left_identity(a in scale_gen()) {
            assert_eq!(Scale::ONE * a, a)
        }

        #[test]
        fn scale_one_scale_mul_right_identity(a in scale_gen()) {
            assert_eq!(a * Scale::ONE, a)
        }

        #[test]
        fn scale_mul_ditributes_over_scale_add(a in scale_gen(), b in scale_gen(), c in scale_gen()) {
            assert_eq!(a * (b + c), a * b + a * c)
        }
    }
}
