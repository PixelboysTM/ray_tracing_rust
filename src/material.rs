use crate::{
    light::PointLight,
    tuples::{
        helpers::{color, colors},
        Tuple,
    },
};

#[derive(Debug, PartialEq, Clone)]
pub struct Material {
    pub color: Tuple,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: color(1, 1, 1),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl Material {
    pub fn lighting(&self, light: &PointLight, point: Tuple, eyev: Tuple, normalv: Tuple) -> Tuple {
        let effective_color = self.color * *light.intensity();

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

        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        light::PointLight,
        tuples::{
            helpers::{color, point, vector},
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

        assert_eq!(m.color, color(1, 1, 1));
        assert!(m.ambient.eps_eq(0.1));
        assert!(m.diffuse.eps_eq(0.9));
        assert!(m.specular.eps_eq(0.9));
        assert!(m.shininess.eps_eq(200.0));
    }

    #[test]
    fn light_eye_between_light_and_surface() {
        let (m, position) = setup();

        let eyev = vector(0, 0, -1);
        let normalv = vector(0, 0, -1);
        let light = PointLight::new(color(1, 1, 1), point(0, 0, -10));

        let result = m.lighting(&light, position, eyev, normalv);
        assert_eq!(result, color(1.9, 1.9, 1.9));
    }

    #[test]
    fn light_eye_between_light_and_surface_45() {
        let (m, position) = setup();

        let eyev = vector(0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = vector(0, 0, -1);
        let light = PointLight::new(color(1, 1, 1), point(0, 0, -10));

        let result = m.lighting(&light, position, eyev, normalv);
        assert_eq!(result, color(1.0, 1.0, 1.0));
    }

    #[test]
    fn light_eye_opposite_surface_45() {
        let (m, position) = setup();

        let eyev = vector(0, 0, -1);
        let normalv = vector(0, 0, -1);
        let light = PointLight::new(color(1, 1, 1), point(0, 10, -10));

        let result = m.lighting(&light, position, eyev, normalv);
        assert_eq!(result, color(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn light_eye_in_path_of_reflection() {
        let (m, position) = setup();

        let eyev = vector(0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = vector(0, 0, -1);
        let light = PointLight::new(color(1, 1, 1), point(0, 10, -10));

        let result = m.lighting(&light, position, eyev, normalv);
        assert_eq!(result, color(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn light_light_behind_surface() {
        let (m, position) = setup();

        let eyev = vector(0, 0, -1);
        let normalv = vector(0, 0, -1);
        let light = PointLight::new(color(1, 1, 1), point(0, 0, 10));

        let result = m.lighting(&light, position, eyev, normalv);
        assert_eq!(result, color(0.1, 0.1, 0.1));
    }
}
