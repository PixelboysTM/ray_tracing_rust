use std::rc::Rc;

use crate::{
    light::PointLight,
    patterns::Pattern,
    shapes::Shape,
    tuples::{
        helpers::{color, colors},
        Tuple,
    },
};

#[derive(Debug)]
pub struct Material {
    pub color: Rc<dyn Pattern>,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: color(1, 1, 1).solid(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.ambient == other.ambient
            && self.diffuse == other.diffuse
            && self.specular == other.specular
            && self.shininess == other.shininess
    }
}

impl Material {
    pub fn clone(m: &Material) -> Material {
        Material {
            color: m.color.clone(),
            ambient: m.ambient,
            diffuse: m.diffuse,
            specular: m.specular,
            shininess: m.shininess,
        }
    }
    pub fn lighting(
        &self,
        object: &dyn Shape,
        light: &PointLight,
        point: Tuple,
        eyev: Tuple,
        normalv: Tuple,
        in_shadow: bool,
    ) -> Tuple {
        let effective_color = self.color.at_object(object, &point) * *light.intensity();

        let lightv = (*light.position() - point).normalized();

        let ambient = effective_color * self.ambient;

        let light_dot_normal = lightv.dot(&normalv);

        let (diffuse, specular) = if light_dot_normal < 0.0 {
            (colors::black(), colors::black())
        } else {
            let diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflectv = (-lightv).reflect(normalv);
            let reflect_dot_eye = reflectv.dot(&eyev);

            let specular = if reflect_dot_eye <= 0.0 {
                colors::black()
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                *light.intensity() * self.specular * factor
            };

            (diffuse, specular)
        };

        ambient
            + if in_shadow {
                colors::black()
            } else {
                diffuse + specular
            }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{
        light::PointLight,
        patterns::StripePattern,
        shapes::Sphere,
        tuples::{
            helpers::{color, colors, point, vector},
            FEquals, Tuple,
        },
    };

    use super::Material;

    fn setup() -> (Material, Tuple) {
        (Material::default(), point(0, 0, 0))
    }

    #[test]
    fn default_material() {
        let m = Material::default();

        // assert_eq!(m.color, color(1, 1, 1));//TODO
        assert!(m.ambient.eps_eq(0.1));
        assert!(m.diffuse.eps_eq(0.9));
        assert!(m.specular.eps_eq(0.9));
        assert!(m.shininess.eps_eq(200.0));
    }

    #[test]
    fn light_eye_between_light_and_surface() {
        let (m, position) = setup();
        let s = Sphere::new();

        let eyev = vector(0, 0, -1);
        let normalv = vector(0, 0, -1);
        let light = PointLight::new(color(1, 1, 1), point(0, 0, -10));

        let result = m.lighting(&s, &light, position, eyev, normalv, false);
        assert_eq!(result, color(1.9, 1.9, 1.9));
    }

    #[test]
    fn light_eye_between_light_and_surface_45() {
        let (m, position) = setup();
        let s = Sphere::new();

        let eyev = vector(0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = vector(0, 0, -1);
        let light = PointLight::new(color(1, 1, 1), point(0, 0, -10));

        let result = m.lighting(&s, &light, position, eyev, normalv, false);
        assert_eq!(result, color(1.0, 1.0, 1.0));
    }

    #[test]
    fn light_eye_opposite_surface_45() {
        let (m, position) = setup();
        let s = Sphere::new();

        let eyev = vector(0, 0, -1);
        let normalv = vector(0, 0, -1);
        let light = PointLight::new(color(1, 1, 1), point(0, 10, -10));

        let result = m.lighting(&s, &light, position, eyev, normalv, false);
        assert_eq!(result, color(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn light_eye_in_path_of_reflection() {
        let (m, position) = setup();
        let s = Sphere::new();

        let eyev = vector(0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = vector(0, 0, -1);
        let light = PointLight::new(color(1, 1, 1), point(0, 10, -10));

        let result = m.lighting(&s, &light, position, eyev, normalv, false);
        assert_eq!(result, color(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn light_light_behind_surface() {
        let (m, position) = setup();
        let s = Sphere::new();

        let eyev = vector(0, 0, -1);
        let normalv = vector(0, 0, -1);
        let light = PointLight::new(color(1, 1, 1), point(0, 0, 10));

        let result = m.lighting(&s, &light, position, eyev, normalv, false);
        assert_eq!(result, color(0.1, 0.1, 0.1));
    }

    #[test]
    fn light_with_surface_in_shadow() {
        let (m, position) = setup();
        let s = Sphere::new();

        let eyev = vector(0, 0, -1);
        let normalv = vector(0, 0, -1);
        let light = PointLight::new(color(1, 1, 1), point(0, 0, -10));
        let in_shadow = true;
        let result = m.lighting(&s, &light, position, eyev, normalv, in_shadow);
        assert_eq!(result, color(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_pattern() {
        let (mut m, position) = setup();
        let s = Sphere::new();

        m.color = Rc::new(StripePattern::new(colors::white(), colors::black()));
        m.ambient = 1.0;
        m.diffuse = 0.0;
        m.specular = 0.0;
        let eyev = vector(0, 0, -1);
        let normalv = vector(0, 0, -1);
        let light = PointLight::new(colors::white(), point(0, 0, -10));

        let c1 = m.lighting(&s, &light, point(0.9, 0, 0), eyev, normalv, false);
        let c2 = m.lighting(&s, &light, point(1.1, 0, 0), eyev, normalv, false);

        assert_eq!(c1, colors::white());
        assert_eq!(c2, colors::black());
    }
}
