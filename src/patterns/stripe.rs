use crate::{
    matrix::helpers::Mat4,
    shapes::Shape,
    tuples::{FEquals, Tuple},
};

use super::Pattern;

#[derive(Debug, PartialEq, Clone)]
pub struct StripePattern {
    a: Tuple,
    b: Tuple,
    transform: Mat4,
}

impl StripePattern {
    pub fn new(a: Tuple, b: Tuple) -> Self {
        StripePattern {
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

impl Pattern for StripePattern {
    fn at(&self, point: &Tuple) -> Tuple {
        if (point.x().floor() % 2.0).eps_eq(0.0) {
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
        shapes::{Shape, Sphere},
        transformation::{scaling, translation},
        tuples::{
            helpers::{colors, point},
            Tuple,
        },
    };

    use super::StripePattern;

    fn setup() -> (Tuple, Tuple) {
        (colors::black(), colors::white())
    }

    #[test]
    fn stripe_pattern() {
        let (black, white) = setup();

        let pattern = StripePattern::new(white, black);
        assert_eq!(pattern.a(), white);
        assert_eq!(pattern.b(), black);
    }

    #[test]
    fn stripe_pattern_constant_y() {
        let (black, white) = setup();

        let pattern = StripePattern::new(white, black);

        assert_eq!(pattern.at(&point(0, 0, 0)), white);
        assert_eq!(pattern.at(&point(0, 1, 0)), white);
        assert_eq!(pattern.at(&point(0, 2, 0)), white);
    }

    #[test]
    fn stripe_pattern_constant_z() {
        let (black, white) = setup();

        let pattern = StripePattern::new(white, black);

        assert_eq!(pattern.at(&point(0, 0, 0)), white);
        assert_eq!(pattern.at(&point(0, 0, 1)), white);
        assert_eq!(pattern.at(&point(0, 0, 2)), white);
    }

    #[test]
    fn stripe_pattern_alternates_x() {
        let (black, white) = setup();

        let pattern = StripePattern::new(white, black);

        assert_eq!(pattern.at(&point(0, 0, 0)), white);
        assert_eq!(pattern.at(&point(0.9, 0, 0)), white);
        assert_eq!(pattern.at(&point(1, 0, 0)), black);
        assert_eq!(pattern.at(&point(-0.1, 0, 0)), black);
        assert_eq!(pattern.at(&point(-1.0, 0, 0)), black);
        assert_eq!(pattern.at(&point(-1.1, 0, 0)), white);
    }

    #[test]
    fn strpies_object_transform() {
        let (black, white) = setup();
        let mut object = Sphere::new();
        object.set_transform(scaling(2.0, 2.0, 2.0));
        let pattern = StripePattern::new(white, black);
        let c = pattern.at_object(&object, &point(1.5, 0, 0));
        assert_eq!(c, white);
    }

    #[test]
    fn strpies_pattern_transform() {
        let (black, white) = setup();
        let object = Sphere::new();
        let mut pattern = StripePattern::new(white, black);
        pattern.set_transform(scaling(2.0, 2.0, 2.0));
        let c = pattern.at_object(&object, &point(1.5, 0, 0));
        assert_eq!(c, white);
    }

    #[test]
    fn strpies_pattern_and_object_transform() {
        let (black, white) = setup();
        let mut object = Sphere::new();
        object.set_transform(scaling(2.0, 2.0, 2.0));
        let mut pattern = StripePattern::new(white, black);
        pattern.set_transform(translation(0.5, 0.0, 0.0));
        let c = pattern.at_object(&object, &point(2.5, 0, 0));
        assert_eq!(c, white);
    }
}
