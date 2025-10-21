use crate::offset::Offset;
use crate::real::Real;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Place {
    pub(super) x: Real,
    pub(super) y: Real,
}

impl std::fmt::Display for Place {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map()
            .entry(&"x", &self.x.to_string())
            .entry(&"y", &self.y.to_string())
            .finish()
    }
}

impl Place {
    pub fn new(x: f64, y: f64) -> Option<Self> {
        let x = Real::from_f64(x)?;
        let y = Real::from_f64(y)?;

        Some(Self { x, y })
    }

    pub fn origin() -> Self {
        Self {
            x: Real::zero(),
            y: Real::zero(),
        }
    }

    pub fn offset_to(self, other: Self) -> Offset {
        other - self
    }
}

///////////
// Addition
///////////

impl std::ops::Add<Offset> for Place {
    type Output = Place;

    fn add(self, rhs: Offset) -> Self::Output {
        let x = self.x + rhs.dx;
        let y = self.y + rhs.dy;

        Self { x, y }
    }
}

impl std::ops::Add<&Offset> for Place {
    type Output = Place;

    fn add(self, rhs: &Offset) -> Self::Output {
        self + rhs.clone()
    }
}

impl std::ops::Add<Offset> for &Place {
    type Output = Place;

    fn add(self, rhs: Offset) -> Self::Output {
        self.clone() + rhs
    }
}

impl std::ops::Add<&Offset> for &Place {
    type Output = Place;

    fn add(self, rhs: &Offset) -> Self::Output {
        self.clone() + rhs.clone()
    }
}

//////////////
// Subtraction
//////////////

impl std::ops::Sub for Place {
    type Output = Offset;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;

        Offset { dx: x, dy: y }
    }
}

impl std::ops::Sub for &Place {
    type Output = Offset;

    fn sub(self, rhs: Self) -> Self::Output {
        self.clone() - rhs.clone()
    }
}

impl std::ops::Sub<&Place> for Place {
    type Output = Offset;

    fn sub(self, rhs: &Place) -> Self::Output {
        self - rhs.clone()
    }
}

impl std::ops::Sub<Place> for &Place {
    type Output = Offset;

    fn sub(self, rhs: Place) -> Self::Output {
        self.clone() - rhs
    }
}

#[cfg(test)]
pub mod gens {
    use proptest::prelude::Strategy;

    use super::Place;
    use crate::real::gens::real;
    use crate::tests::sampler;

    pub fn place() -> impl Strategy<Value = Place> {
        (real(), real()).prop_map(|(x, y)| Place { x, y })
    }

    #[test]
    #[ignore = "just examples of Place"]
    fn print_places() {
        sampler(place()).take(10).for_each(|p| {
            println!("Place: {p:#}");
        });
    }
}

#[cfg(test)]
mod tests {
    use proptest::array::uniform2;
    use proptest::proptest;

    use crate::offset::gens::offset;
    use crate::offset::Offset;
    use crate::place::gens::place;

    proptest! {
        #[test]
        fn offset_zero_place_add_right_identity(p in place()) {
            assert_eq!(&p + Offset::zero(), p)
        }

        #[test]
        fn place_add_offset_add_associative(p in place(), [a, b] in uniform2(offset())) {
            assert_eq!((&p + &a) + &b, &p + (&a + &b))
        }

        #[test]
        fn place_add_place_sub([p, q] in uniform2(place())) {
            assert_eq!(&p + (&q - &p), q)
        }
    }
}
