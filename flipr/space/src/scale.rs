use crate::real::Real;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Scale(pub(super) Real);

impl std::fmt::Display for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Scale {
    pub fn one() -> Self {
        Self(Real::one())
    }

    pub fn zero() -> Self {
        Self(Real::zero())
    }
}

/////////////////
// Multiplication
/////////////////

impl std::ops::Mul for Scale {
    type Output = Scale;

    fn mul(self, rhs: Scale) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl std::ops::Mul for &Scale {
    type Output = Scale;

    fn mul(self, rhs: &Scale) -> Self::Output {
        self.clone() * rhs.clone()
    }
}

impl std::ops::Mul<&Scale> for Scale {
    type Output = Scale;

    fn mul(self, rhs: &Scale) -> Self::Output {
        self * rhs.clone()
    }
}

impl std::ops::Mul<Scale> for &Scale {
    type Output = Scale;

    fn mul(self, rhs: Scale) -> Self::Output {
        self.clone() * rhs
    }
}

///////////
// Addition
///////////

impl std::ops::Add for Scale {
    type Output = Scale;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Add for &Scale {
    type Output = Scale;

    fn add(self, rhs: Self) -> Self::Output {
        self.clone() + rhs.clone()
    }
}

impl std::ops::Add<&Scale> for Scale {
    type Output = Scale;

    fn add(self, rhs: &Scale) -> Self::Output {
        self + rhs.clone()
    }
}

impl std::ops::Add<Scale> for &Scale {
    type Output = Scale;

    fn add(self, rhs: Scale) -> Self::Output {
        self.clone() + rhs
    }
}

///////////
// Negation
///////////

impl std::ops::Neg for Scale {
    type Output = Scale;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl std::ops::Neg for &Scale {
    type Output = Scale;

    fn neg(self) -> Self::Output {
        -self.clone()
    }
}

#[cfg(test)]
pub mod gens {
    use proptest::prelude::Strategy;

    use super::Scale;
    use crate::real::gens::real;
    use crate::tests::sampler;

    /// Generates arbitrary Scale values for testing.
    pub fn scale() -> impl Strategy<Value = Scale> {
        real().prop_map(|d| Scale(d))
    }

    #[test]
    #[ignore = "just examples of Scale"]
    fn print_scales() {
        sampler(scale()).take(10).for_each(|r| {
            println!("Scale: {:#}", r);
        });
    }
}

#[cfg(test)]
mod tests {
    use proptest::array::{uniform2, uniform3};
    use proptest::{prop_assert_eq, proptest};

    use super::gens::scale;
    use super::Scale;

    proptest! {
        #[test]
        fn scale_add_associative([m, n, o] in uniform3(scale())) {
            prop_assert_eq!(&m + (&n + &o), (&m + &n) + &o);
        }

        #[test]
        fn scale_add_commutative([m, n] in uniform2(scale())) {
            prop_assert_eq!(&m + &n, &n + &m);
        }

        #[test]
        fn scale_zero_scale_add_left_identity(m in scale()) {
            prop_assert_eq!(Scale::zero() + &m, m);
        }

        #[test]
        fn scale_zero_scale_add_right_identity(m in scale()) {
            prop_assert_eq!(&m + Scale::zero(), m);
        }

        #[test]
        fn scale_add_inverse(m in scale()) {
            prop_assert_eq!(&m + (-&m), Scale::zero());
        }

        #[test]
        fn scale_mul_associative([m, n, o] in uniform3(scale())) {
            prop_assert_eq!(&m * (&n * &o), (&m * &n) * &o);
        }

        #[test]
        fn scale_mul_commutative([m, n] in uniform2(scale())) {
            prop_assert_eq!(&m * &n, &n * &m);
        }

        #[test]
        fn scale_zero_scale_mul_left_anihilator(m in scale()) {
            prop_assert_eq!(Scale::zero() * &m, Scale::zero());
        }

        #[test]
        fn scale_zero_scale_mul_right_anihilator(m in scale()) {
            prop_assert_eq!(&m * Scale::zero(), Scale::zero());
        }

        #[test]
        fn scale_one_scale_mul_left_identity(m in scale()) {
            prop_assert_eq!(Scale::one() * &m, m);
        }

        #[test]
        fn scale_one_scale_mul_right_identity(m in scale()) {
            prop_assert_eq!(&m * Scale::one(), m);
        }

        #[test]
        fn scale_mul_distributes_over_scale_add([m, n, o] in uniform3(scale())) {
            prop_assert_eq!(&m * (&n + &o), &m * &n + &m * &o);
        }
    }
}
