use num::rational::Ratio;
use num::{BigInt, ToPrimitive};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Real(Ratio<BigInt>);

impl std::fmt::Display for Real {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Real {
    pub fn one() -> Self {
        Self(Ratio::from_integer(BigInt::from(1)))
    }

    pub fn zero() -> Self {
        Self(Ratio::from_integer(BigInt::from(0)))
    }

    pub fn from_f64(value: f64) -> Option<Self> {
        Ratio::from_float(value).map(|r| Self(r))
    }

    pub fn to_f64(&self) -> Option<f64> {
        self.0.to_f64()
    }

    pub fn sin(&self) -> Self {
        Self::from_f64(
            self.0
                .to_f64()
                .expect("current implementation of sin can work only with finite f64")
                .sin(),
        )
        .expect("sin of finite f64 should produce finite f64")
    }

    pub fn cos(&self) -> Self {
        Self::from_f64(
            self.0
                .to_f64()
                .expect("current implementation of cos can work only with finite f64")
                .cos(),
        )
        .expect("cos of finite f64 should produce finite f64")
    }
}

///////////
// Addition
///////////

impl std::ops::Add for Real {
    type Output = Real;

    fn add(self, rhs: Self) -> Self::Output {
        Real(self.0 + rhs.0)
    }
}

impl std::ops::Add for &Real {
    type Output = Real;

    fn add(self, rhs: Self) -> Self::Output {
        self.clone() + rhs.clone()
    }
}

impl std::ops::Add<&Real> for Real {
    type Output = Real;

    fn add(self, rhs: &Real) -> Self::Output {
        self + rhs.clone()
    }
}

impl std::ops::Add<Real> for &Real {
    type Output = Real;

    fn add(self, rhs: Real) -> Self::Output {
        self.clone() + rhs
    }
}

//////////////
// Subtraction
//////////////

impl std::ops::Sub for Real {
    type Output = Real;

    fn sub(self, rhs: Self) -> Self::Output {
        Real(self.0 - rhs.0)
    }
}

impl std::ops::Sub for &Real {
    type Output = Real;

    fn sub(self, rhs: Self) -> Self::Output {
        self.clone() - rhs.clone()
    }
}

impl std::ops::Sub<&Real> for Real {
    type Output = Real;

    fn sub(self, rhs: &Real) -> Self::Output {
        self - rhs.clone()
    }
}

impl std::ops::Sub<Real> for &Real {
    type Output = Real;

    fn sub(self, rhs: Real) -> Self::Output {
        self.clone() - rhs
    }
}

/////////////////
// Multiplication
/////////////////

impl std::ops::Mul for Real {
    type Output = Real;

    fn mul(self, rhs: Self) -> Self::Output {
        Real(self.0 * rhs.0)
    }
}

impl std::ops::Mul for &Real {
    type Output = Real;

    fn mul(self, rhs: Self) -> Self::Output {
        self.clone() * rhs.clone()
    }
}

impl std::ops::Mul<&Real> for Real {
    type Output = Real;

    fn mul(self, rhs: &Real) -> Self::Output {
        self * rhs.clone()
    }
}

impl std::ops::Mul<Real> for &Real {
    type Output = Real;

    fn mul(self, rhs: Real) -> Self::Output {
        self.clone() * rhs
    }
}

///////////
// Negation
///////////

impl std::ops::Neg for Real {
    type Output = Real;

    fn neg(self) -> Self::Output {
        Real(-self.0)
    }
}

impl std::ops::Neg for &Real {
    type Output = Real;

    fn neg(self) -> Self::Output {
        Real(-self.0.clone())
    }
}

///////////
// Division
///////////

impl std::ops::Div for Real {
    type Output = Real;

    fn div(self, rhs: Self) -> Self::Output {
        Real(self.0 / rhs.0)
    }
}

impl std::ops::Div for &Real {
    type Output = Real;

    fn div(self, rhs: Self) -> Self::Output {
        self.clone() / rhs.clone()
    }
}

impl std::ops::Div<&Real> for Real {
    type Output = Real;

    fn div(self, rhs: &Real) -> Self::Output {
        self / rhs.clone()
    }
}

impl std::ops::Div<Real> for &Real {
    type Output = Real;

    fn div(self, rhs: Real) -> Self::Output {
        self.clone() / rhs
    }
}

#[cfg(test)]
pub mod gens {
    use proptest::prelude::Strategy;
    use proptest::{prop_assert, prop_assume, proptest};

    use super::Real;
    use crate::tests::sampler;

    /// Generates arbitrary Real values for testing.
    pub fn real() -> impl Strategy<Value = Real> {
        (proptest::num::f64::NORMAL
            | proptest::num::f64::NEGATIVE
            | proptest::num::f64::POSITIVE
            | proptest::num::f64::ZERO)
            .prop_map(|f| Real::from_f64(f).expect("any finite f64 should be a valid Real"))
    }

    #[test]
    #[ignore = "just examples of Real"]
    fn print_reals() {
        sampler(real()).take(10).for_each(|r| {
            println!("Real: {r:#}");
        });
    }

    proptest! {
        #[test]
        fn any_finite_f64_is_a_valid_real(value : f64) {
            prop_assume!(value.is_finite());
            prop_assert!(Real::from_f64(value).is_some());
        }
    }
}

#[cfg(test)]
mod tests {
    use proptest::array::{uniform2, uniform3};
    use proptest::{prop_assert_eq, prop_assume, proptest};

    use super::gens::real;
    use super::Real;

    proptest! {
        #[test]
        fn zero_is_additive_right_identity(a in real()) {
            prop_assert_eq!(&a + Real::zero(), a);
        }

        #[test]
        fn zero_is_additive_left_identity(a in real()) {
            prop_assert_eq!(Real::zero() + &a, a);
        }

        #[test]
        fn addition_is_commutative((a, b) in (real(), real())) {
            prop_assert_eq!(&a + &b, &b + &a);
        }

        #[test]
        fn addition_is_associative([a, b, c] in uniform3(real())) {
            prop_assert_eq!((&a + &b) + &c, &a + (&b + &c));
        }

        #[test]
        fn one_is_multiplicative_right_identity(a in real()) {
            prop_assert_eq!(&a * Real::one(), a);
        }

        #[test]
        fn one_is_multiplicative_left_identity(a in real()) {
            prop_assert_eq!(Real::one() * &a, a);
        }

        #[test]
        fn zero_is_multiplicative_right_annihilator(a in real()) {
            prop_assert_eq!(&a * Real::zero(), Real::zero());
        }

        #[test]
        fn zero_is_multiplicative_left_annihilator(a in real()) {
            prop_assert_eq!(Real::zero() * &a, Real::zero());
        }

        #[test]
        fn multiplication_is_commutative((a, b) in (real(), real())) {
            prop_assert_eq!(&a * &b, &b * &a);
        }

        #[test]
        fn multiplication_is_associative([a, b, c] in uniform3(real())) {
            prop_assert_eq!((&a * &b) * &c, &a * (&b * &c));
        }

        #[test]
        fn multiplication_distributes_over_addition([a, b, c] in uniform3(real())) {
            prop_assert_eq!(&a * (&b + &c), &a * &b + &a * &c);
        }

        #[test]
        fn negation_is_additive_inverse(a in real()) {
            prop_assert_eq!(&a + -&a, Real::zero());
        }

        #[test]
        fn subtraction_is_addition_of_inverse([a, b] in uniform2(real())) {
            prop_assert_eq!(&a - &b, &a + -&b);
        }

        #[test]
        fn negation_is_involutive(a in real()) {
            prop_assert_eq!(-(-&a), a);
        }

        #[test]
        fn division_by_nonzero_is_valid([a, b] in uniform2(real())) {
            prop_assume!(b != Real::zero());
            let _ = a / b;
        }

        #[test]
        fn division_panics_on_division_by_zero(a in real()) {
            std::panic::set_hook(Box::new(|_: &std::panic::PanicHookInfo| {}));
            prop_assert_eq!(std::panic::catch_unwind(|| {
                let _ = a / Real::zero();
            })
            .is_err(), true);
        }
    }
}
