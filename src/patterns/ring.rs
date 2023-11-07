use crate::{
    matrix::helpers::Mat4,
    tuples::{FEquals, Tuple},
};

use super::Pattern;

#[derive(Debug)]
pub struct RingPattern {
    a: Tuple,
    b: Tuple,
    transform: Mat4,
}
impl RingPattern {
    pub fn new(a: Tuple, b: Tuple) -> Self {
        RingPattern {
            a,
            b,
            transform: Mat4::identity(),
        }
    }
    pub fn a(&self) -> Tuple {
        self.a
    }
    pub fn b(&self) -> Tuple {
        self.b
    }
}

impl Pattern for RingPattern {
    fn at(&self, point: &Tuple) -> Tuple {
        if ((point.x() * point.x() + point.z() * point.z()).sqrt() % 2.0).eps_eq(0.0) {
            self.a
        } else {
            self.b
        }
    }
    fn transform(&self) -> &Mat4 {
        &self.transform
    }
    fn set_transform(&mut self, new_transform: Mat4) {
        self.transform = new_transform;
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        patterns::Pattern,
        tuples::helpers::{colors, point},
    };

    use super::RingPattern;

    #[test]
    fn ring_pattern() {
        let pattern = RingPattern::new(colors::white(), colors::black());

        assert_eq!(pattern.at(&point(0, 0, 0)), colors::white());
        assert_eq!(pattern.at(&point(1, 0, 0)), colors::black());
        assert_eq!(pattern.at(&point(0, 0, 1)), colors::black());
        assert_eq!(pattern.at(&point(0.708, 0, 0.708)), colors::black());
    }
}
