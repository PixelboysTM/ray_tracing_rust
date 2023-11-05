use crate::tuples::Tuple;

#[derive(Debug, PartialEq)]
pub struct PointLight {
    intensity: Tuple,
    position: Tuple,
}

impl PointLight {
    pub fn new(intensity: Tuple, position: Tuple) -> PointLight {
        PointLight {
            intensity,
            position,
        }
    }
    pub fn intensity(&self) -> &Tuple {
        &self.intensity
    }
    pub fn position(&self) -> &Tuple {
        &self.position
    }
}

#[cfg(test)]
mod tests {
    use crate::tuples::helpers::{color, point};

    use super::PointLight;

    #[test]
    fn light_has_position_and_intensity() {
        let intensity = color(1, 1, 1);
        let position = point(0, 0, 0);
        let light = PointLight::new(intensity, position);
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
