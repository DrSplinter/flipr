use crate::real::Real;
use crate::scale::Scale;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Offset {
    pub(super) dx: Real,
    pub(super) dy: Real,
}

impl std::fmt::Display for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map()
            .entry(&"dx", &self.dx.to_string())
            .entry(&"dy", &self.dy.to_string())
            .finish()
    }
}

impl Offset {
    pub fn zero() -> Self {
        Self {
            dx: Real::zero(),
            dy: Real::zero(),
        }
    }

    pub fn new(x: f64, y: f64) -> Option<Self> {
        let dx = Real::from_f64(x)?;
        let dy = Real::from_f64(y)?;

        Some(Self { dx, dy })
    }
}

///////////
// Addition
///////////

impl std::ops::Add for Offset {
    type Output = Offset;

    fn add(self, rhs: Self) -> Self::Output {
        let dx = self.dx + rhs.dx;
        let dy = self.dy + rhs.dy;

        Self { dx, dy }
    }
}

impl std::ops::Add for &Offset {
    type Output = Offset;

    fn add(self, rhs: Self) -> Self::Output {
        self.clone() + rhs.clone()
    }
}

impl std::ops::Add<&Offset> for Offset {
    type Output = Offset;

    fn add(self, rhs: &Offset) -> Self::Output {
        self + rhs.clone()
    }
}

impl std::ops::Add<Offset> for &Offset {
    type Output = Offset;

    fn add(self, rhs: Offset) -> Self::Output {
        self.clone() + rhs
    }
}

///////////
// Negation
///////////

impl std::ops::Neg for Offset {
    type Output = Offset;

    fn neg(self) -> Self::Output {
        let dx = -self.dx;
        let dy = -self.dy;

        Self { dx, dy }
    }
}

impl std::ops::Neg for &Offset {
    type Output = Offset;

    fn neg(self) -> Self::Output {
        -self.clone()
    }
}

/////////////////
// Multiplication
/////////////////

impl std::ops::Mul<Scale> for Offset {
    type Output = Offset;

    fn mul(self, rhs: Scale) -> Self::Output {
        let dx = self.dx * &rhs.0;
        let dy = self.dy * rhs.0;

        Self { dx, dy }
    }
}

impl std::ops::Mul<&Scale> for Offset {
    type Output = Offset;

    fn mul(self, rhs: &Scale) -> Self::Output {
        self * rhs.clone()
    }
}

impl std::ops::Mul<Scale> for &Offset {
    type Output = Offset;

    fn mul(self, rhs: Scale) -> Self::Output {
        self.clone() * rhs
    }
}

impl std::ops::Mul<&Scale> for &Offset {
    type Output = Offset;

    fn mul(self, rhs: &Scale) -> Self::Output {
        self.clone() * rhs.clone()
    }
}

#[cfg(test)]
pub mod gens {
    use proptest::prelude::Strategy;

    use crate::offset::Offset;
    use crate::real::gens::real;
    use crate::tests::sampler;

    pub fn offset() -> impl Strategy<Value = Offset> {
        (real(), real()).prop_map(|(dx, dy)| Offset { dx, dy })
    }

    #[test]
    #[ignore = "just examples of Offset"]
    fn print_offsets() {
        sampler(offset()).take(10).for_each(|a| {
            println!("Offset: {a:#}");
        });
    }
}

#[cfg(test)]
mod tests {
    use proptest::array::{uniform2, uniform3};
    use proptest::proptest;

    use super::gens::offset;
    use super::*;
    use crate::scale::gens::scale;

    proptest! {
        #[test]
        fn offset_add_associative([a, b, c] in uniform3(offset())) {
            assert_eq!(&a + (&b + &c), (&a + &b) + &c)
        }

        #[test]
        fn offset_add_commutative([a, b] in uniform2(offset())) {
            assert_eq!(&a + &b, &b + &a)
        }

            #[test]
        fn offset_zero_offset_add_left_identity(a in offset()) {
            assert_eq!(Offset::zero() + &a, a)
        }

        #[test]
        fn offset_zero_offset_add_right_identity(a in offset()) {
            assert_eq!(&a + Offset::zero(), a)
        }

        #[test]
        fn offset_add_inverse(a in offset()) {
            assert_eq!(&a + -&a, Offset::zero())
        }

        #[test]
        fn offset_mul_scale_mul_associative(a in offset(), [m, n] in uniform2(scale())) {
            assert_eq!((&a * &m) * &n, &a * (&m * &n))
        }

        #[test]
        fn scale_one_offset_mul_right_identity(a in offset()) {
            assert_eq!(&a * Scale::one(), a);
        }

        #[test]
        fn offset_mul_distributive_over_offset_add([a, b] in uniform2(offset()), m in scale()) {
            assert_eq!((&a + &b) * &m, &a * &m + &b * &m)
        }

        #[test]
        fn offset_mul_distributive_over_scale_add(a in offset(), [m, n] in uniform2(scale())) {
            assert_eq!(&a * (&m + &n), &a * &m + &a * &n)
        }
    }
}
