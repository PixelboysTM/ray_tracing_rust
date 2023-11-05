use crate::{
    light::PointLight,
    material::Material,
    ray::{Computations, Intersection, Intersections, Ray},
    sphere::Sphere,
    transformation::scaling,
    tuples::{
        helpers::{color, colors, point},
        Tuple,
    },
};

pub struct World {
    objects: Vec<Sphere>,
    light: Option<PointLight>,
}

impl World {
    pub fn new() -> Self {
        World {
            objects: vec![],
            light: None,
        }
    }
    pub fn objetcs(&self) -> &Vec<Sphere> {
        &self.objects
    }
    pub fn objetcs_mut(&mut self) -> &mut Vec<Sphere> {
        &mut self.objects
    }
    pub fn light(&self) -> Option<&PointLight> {
        self.light.as_ref()
    }
    pub fn light_mut(&mut self) -> Option<&mut PointLight> {
        self.light.as_mut()
    }
    pub fn set_light(&mut self, light: Option<PointLight>) {
        self.light = light;
    }
    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut xs = Vec::new();

        for obj in &self.objects {
            xs.append(&mut obj.intersect(ray));
        }

        xs.sort_by(|a, b| a.t().partial_cmp(&b.t()).unwrap());

        xs
    }

    pub fn shade_hit(&self, comps: &Computations) -> Tuple {
        comps.object.material().lighting(
            self.light().unwrap(),
            comps.point,
            comps.eyev,
            comps.normalv,
        )
    }

    pub fn color_at(&self, ray: &Ray) -> Tuple {
        let xs = self.intersect(ray);
        if let Some(hit) = xs.hit() {
            let comps = hit.prepare_comps(ray);
            self.shade_hit(&comps)
        } else {
            colors::black()
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let mut s1 = Sphere::new();
        s1.set_material(Material {
            color: color(0.8, 1.0, 0.6),
            diffuse: 0.7,
            specular: 0.2,
            ..Default::default()
        });
        let mut s2 = Sphere::new();
        s2.set_transform(scaling(0.5, 0.5, 0.5));

        Self {
            light: Some(PointLight::new(color(1, 1, 1), point(-10, 10, -10))),
            objects: vec![s1, s2],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        light::PointLight,
        material::Material,
        ray::{Intersection, Ray},
        sphere::Sphere,
        transformation::scaling,
        tuples::{
            helpers::{color, point, vector},
            FEquals,
        },
    };

    use super::World;

    #[test]
    fn creating_a_world() {
        let w = World::new();
        assert_eq!(w.objetcs().len(), 0);
        assert_eq!(w.light(), None);
    }

    #[test]
    fn the_default_world() {
        let light = PointLight::new(color(1, 1, 1), point(-10, 10, -10));
        let mut s1 = Sphere::new();
        s1.set_material(Material {
            color: color(0.8, 1.0, 0.6),
            diffuse: 0.7,
            specular: 0.2,
            ..Default::default()
        });
        let mut s2 = Sphere::new();
        s2.set_transform(scaling(0.5, 0.5, 0.5));

        let w = World::default();
        assert_eq!(w.light().unwrap(), &light);
        assert!(w.objetcs().contains(&s1));
        assert!(w.objetcs().contains(&s2));
    }

    #[test]
    fn intersecting_world_ray() {
        let w = World::default();

        let r = Ray::new(point(0, 0, -5), vector(0, 0, 1));

        let xs = w.intersect(&r);
        assert_eq!(xs.len(), 4);
        assert!(xs[0].t().eps_eq(4.0));
        assert!(xs[1].t().eps_eq(4.5));
        assert!(xs[2].t().eps_eq(5.5));
        assert!(xs[3].t().eps_eq(6.0));
    }

    #[test]
    fn shading_intersection() {
        let w = World::default();

        let r = Ray::new(point(0, 0, -5), vector(0, 0, 1));

        let shape = &w.objetcs()[0];

        let i = Intersection::new(4.0, shape);

        let comps = i.prepare_comps(&r);

        let c = w.shade_hit(&comps);
        assert_eq!(c, color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_intersection_from_inside() {
        let mut w = World::default();
        w.light = Some(PointLight::new(color(1, 1, 1), point(0, 0.25, 0)));

        let r = Ray::new(point(0, 0, 0), vector(0, 0, 1));

        let shape = &w.objetcs()[1];

        let i = Intersection::new(0.5, shape);

        let comps = i.prepare_comps(&r);

        let c = w.shade_hit(&comps);
        assert_eq!(c, color(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn color_ray_miss() {
        let w = World::default();

        let r = Ray::new(point(0, 0, -5), vector(0, 1, 0));

        let c = w.color_at(&r);
        assert_eq!(c, color(0, 0, 0));
    }

    #[test]
    fn color_ray_hits() {
        let w = World::default();

        let r = Ray::new(point(0, 0, -5), vector(0, 0, 1));

        let c = w.color_at(&r);
        assert_eq!(c, color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_ray_behind() {
        let mut w = World::default();

        {
            let outer = &mut w.objetcs_mut()[0];
            outer.material_mut().ambient = 1.0;
        }
        {
            let inner = &mut w.objetcs_mut()[1];
            inner.material_mut().ambient = 1.0;
        }
        let inner = &w.objetcs()[1];

        let r = Ray::new(point(0, 0, 0.75), vector(0, 0, -1));

        let c = w.color_at(&r);
        assert_eq!(c, inner.material().color);
    }
}
