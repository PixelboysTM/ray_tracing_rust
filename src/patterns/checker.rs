use crate::{
    matrix::helpers::Mat4,
    tuples::{FEquals, Tuple},
};

use super::Pattern;

#[derive(Debug)]
pub struct CheckerPattern {
    a: Tuple,
    b: Tuple,
    transform: Mat4,
}
impl CheckerPattern {
    pub fn new(a: Tuple, b: Tuple) -> Self {
        CheckerPattern {
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

impl Pattern for CheckerPattern {
    fn at(&self, point: &Tuple) -> Tuple {
        if ((point.x().floor() + point.y().floor() + point.z().floor()) % 2.0).eps_eq(0.0) {
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
        tuples::helpers::{color, colors, point},
    };

    use super::CheckerPattern;

    #[test]
    fn repeat_x() {
        let pattern = CheckerPattern::new(colors::white(), colors::black());

        assert_eq!(pattern.at(&point(0, 0, 0)), colors::white());
        assert_eq!(pattern.at(&point(0.99, 0, 0)), colors::white());
        assert_eq!(pattern.at(&point(1.01, 0, 0)), colors::black());
    }

    #[test]
    fn repeat_y() {
        let pattern = CheckerPattern::new(colors::white(), colors::black());

        assert_eq!(pattern.at(&point(0, 0, 0)), colors::white());
        assert_eq!(pattern.at(&point(0, 0.99, 0)), colors::white());
        assert_eq!(pattern.at(&point(0, 1.01, 0)), colors::black());
    }

    #[test]
    fn repeat_z() {
        let pattern = CheckerPattern::new(colors::white(), colors::black());

        assert_eq!(pattern.at(&point(0, 0, 0)), colors::white());
        assert_eq!(pattern.at(&point(0, 0, 0.99)), colors::white());
        assert_eq!(pattern.at(&point(0, 0, 1.01)), colors::black());
    }
}
