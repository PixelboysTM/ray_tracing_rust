use crate::{
    material::Material,
    matrix::helpers::Mat4,
    ray::Intersection,
    tuples::{helpers::vector, EPSILON},
};

use super::Shape;

#[derive(Debug)]
pub struct Plane {
    transform: Mat4,
    material: Material,
}

impl Plane {
    pub fn new() -> Plane {
        Plane {
            transform: Mat4::identity(),
            material: Material::default(),
        }
    }
}

impl Shape for Plane {
    fn transform(&self) -> &Mat4 {
        &self.transform
    }

    fn set_transform(&mut self, new_transform: Mat4) {
        self.transform = new_transform;
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn set_material(&mut self, new_material: Material) {
        self.material = new_material;
    }

    fn local_intersect(&self, ray: &crate::ray::Ray) -> Vec<crate::ray::Intersection> {
        if ray.direction().y().abs() < EPSILON {
            Vec::new()
        } else {
            vec![Intersection::new(
                -ray.origin().y() / ray.direction().y(),
                self,
            )]
        }
    }

    fn local_normal_at(&self, p: crate::tuples::Tuple) -> crate::tuples::Tuple {
        vector::up()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ray::Ray,
        shapes::Shape,
        tuples::{
            helpers::{point, vector},
            FEquals,
        },
    };

    use super::Plane;

    #[test]
    fn normal_is_const() {
        let p = Plane::new();
        let n1 = p.local_normal_at(point(0, 0, 0));
        let n2 = p.local_normal_at(point(10, 0, -10));
        let n3 = p.local_normal_at(point(-5, 0, 150));

        assert_eq!(n1, vector(0, 1, 0));
        assert_eq!(n2, vector(0, 1, 0));
        assert_eq!(n3, vector(0, 1, 0));
    }

    #[test]
    fn ray_parrallel() {
        let p = Plane::new();
        let r = Ray::new(point(0, 10, 0), vector(0, 0, 1));
        let xs = p.local_intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_coplanar() {
        let p = Plane::new();
        let r = Ray::new(point(0, 0, 0), vector(0, 0, 1));
        let xs = p.local_intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_from_above() {
        let p = Plane::new();
        let r = Ray::new(point(0, 1, 0), vector(0, -1, 0));
        let xs = p.local_intersect(&r);
        assert_eq!(xs.len(), 1);
        assert!(xs[0].t().eps_eq(1.0));
        // assert!(xs[0].object(), p); //TODO
    }

    #[test]
    fn ray_from_below() {
        let p = Plane::new();
        let r = Ray::new(point(0, -1, 0), vector(0, 1, 0));
        let xs = p.local_intersect(&r);
        assert_eq!(xs.len(), 1);
        assert!(xs[0].t().eps_eq(1.0));
        // assert!(xs[0].object(), p); //TODO
    }
}
