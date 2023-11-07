use crate::{matrix::helpers::Mat4, tuples::Tuple};

use super::Pattern;

#[derive(Debug)]
pub struct GradientPattern {
    a: Tuple,
    b: Tuple,
    transform: Mat4,
}
impl GradientPattern {
    pub fn new(a: Tuple, b: Tuple) -> Self {
        GradientPattern {
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

impl Pattern for GradientPattern {
    fn at(&self, point: &Tuple) -> Tuple {
        let distance = self.a - self.b;
        let fraction = point.x() - point.x().floor();

        self.a - distance * fraction
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

    use super::GradientPattern;

    #[test]
    fn gradient_lerp() {
        let pattern = GradientPattern::new(colors::white(), colors::black());

        assert_eq!(pattern.at(&point(0, 0, 0)), colors::white());
        assert_eq!(pattern.at(&point(0.25, 0, 0)), color(0.75, 0.75, 0.75));
        assert_eq!(pattern.at(&point(0.5, 0, 0)), color(0.5, 0.5, 0.5));
        assert_eq!(pattern.at(&point(0.75, 0, 0)), color(0.25, 0.25, 0.25));
    }
}
