mod checker;
mod gradient;
mod ring;
mod solid;
mod stripe;
use std::fmt::Debug;

pub use checker::CheckerPattern;
pub use gradient::GradientPattern;
pub use ring::RingPattern;
pub use solid::Solid;
pub use stripe::StripePattern;

use crate::{
    matrix::helpers::Mat4,
    shapes::Shape,
    tuples::{helpers::color, Tuple},
};

pub trait Pattern: Debug {
    fn at(&self, point: &Tuple) -> Tuple;
    fn transform(&self) -> &Mat4;
    fn set_transform(&mut self, new_transform: Mat4);

    fn at_object(&self, object: &dyn Shape, point: &Tuple) -> Tuple {
        let object_point = object.transform().inverse() * *point;
        let pattern_point = self.transform().inverse() * object_point;

        self.at(&pattern_point)
    }
}

#[derive(Debug, Clone)]
pub struct TestPattern {
    m: Mat4,
}

impl TestPattern {
    pub fn new() -> Self {
        Self {
            m: Mat4::identity(),
        }
    }
}

impl Pattern for TestPattern {
    fn at(&self, point: &Tuple) -> Tuple {
        color(point.x(), point.y(), point.z())
    }

    fn transform(&self) -> &Mat4 {
        &self.m
    }

    fn set_transform(&mut self, new_transform: Mat4) {
        self.m = new_transform;
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        matrix::helpers::Mat4,
        patterns::Pattern,
        shapes::{Shape, Sphere},
        transformation::{scaling, translation},
        tuples::helpers::{color, point},
    };

    use super::TestPattern;

    #[test]
    fn default_pattern_transform() {
        let pattern = TestPattern::new();
        assert_eq!(pattern.transform(), &Mat4::identity());
    }

    #[test]
    fn assigning_a_transform() {
        let mut pattern = TestPattern::new();
        pattern.set_transform(translation(1.0, 2.0, 3.0));
        assert_eq!(pattern.transform(), &translation(1.0, 2.0, 3.0));
    }

    #[test]
    fn object_transform() {
        let mut object = Sphere::new();
        object.set_transform(scaling(2.0, 2.0, 2.0));
        let pattern = TestPattern::new();
        let c = pattern.at_object(&object, &point(2.0, 3.0, 4.0));
        assert_eq!(c, color(1, 1.5, 2));
    }

    #[test]
    fn pattern_transform() {
        let object = Sphere::new();
        let mut pattern = TestPattern::new();
        pattern.set_transform(scaling(2.0, 2.0, 2.0));
        let c = pattern.at_object(&object, &point(2, 3, 4));
        assert_eq!(c, color(1, 1.5, 2));
    }

    #[test]
    fn pattern_and_object_transform() {
        let mut object = Sphere::new();
        object.set_transform(scaling(2.0, 2.0, 2.0));
        let mut pattern = TestPattern::new();
        pattern.set_transform(translation(0.5, 1.0, 1.5));
        let c = pattern.at_object(&object, &point(2.5, 3, 3.5));
        assert_eq!(c, color(0.75, 0.5, 0.25));
    }
}
