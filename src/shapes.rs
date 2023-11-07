mod plane;
pub use plane::Plane;
mod sphere;
pub use sphere::Sphere;

use std::{fmt::Debug, sync::Mutex};

use crate::{
    material::Material,
    matrix::helpers::Mat4,
    ray::{Intersection, Ray},
    tuples::Tuple,
};

pub trait Shape: Debug {
    fn transform(&self) -> &Mat4;
    fn set_transform(&mut self, new_transform: Mat4);
    fn material(&self) -> &Material;
    fn material_mut(&mut self) -> &mut Material;
    fn set_material(&mut self, new_material: Material);
    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection>;
    fn local_normal_at(&self, p: Tuple) -> Tuple;

    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let object_ray = ray.transform(&self.transform().inverse());
        self.local_intersect(&object_ray)
    }

    fn normal_at(&self, p: Tuple) -> Tuple {
        let object_point = self.transform().inverse() * p;
        let object_normal = self.local_normal_at(object_point);
        let world_normal = self.transform().inverse().transpose() * object_normal;

        (Tuple::vector(world_normal.x(), world_normal.y(), world_normal.z())).normalized()
    }
}

#[derive(Debug)]
pub struct TestShape {
    transformation: Mat4,
    material: Material,
    saved_ray: Mutex<Option<Ray>>,
}

impl PartialEq for TestShape {
    fn eq(&self, other: &Self) -> bool {
        self.transformation == other.transformation && self.material == other.material
    }
}

impl TestShape {
    pub fn new() -> TestShape {
        Self {
            transformation: Mat4::identity(),
            material: Material::default(),
            saved_ray: Mutex::new(None),
        }
    }
}

impl Shape for TestShape {
    fn transform(&self) -> &Mat4 {
        &self.transformation
    }

    fn set_transform(&mut self, new_transform: Mat4) {
        self.transformation = new_transform;
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

    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut m = self.saved_ray.lock().unwrap();
        *m = Some(Ray::new(ray.origin(), ray.direction()));

        vec![]
    }

    fn local_normal_at(&self, p: Tuple) -> Tuple {
        Tuple::vector(p.x(), p.y(), p.z())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        material::Material,
        matrix::helpers::Mat4,
        ray::Ray,
        shapes::Shape,
        transformation::{helper::TransformationBuilder, scaling, translation, PI},
        tuples::helpers::{point, vector},
    };

    use super::TestShape;

    #[test]
    fn default_transform() {
        let s = TestShape::new();
        assert_eq!(s.transform(), &Mat4::identity());
    }

    #[test]
    fn asigning_transfomr() {
        let mut s = TestShape::new();
        s.set_transform(translation(2.0, 3.0, 4.0));

        assert_eq!(s.transform(), &translation(2.0, 3.0, 4.0));
    }

    #[test]
    fn default_mat() {
        let s = TestShape::new();
        let m = s.material();
        assert_eq!(m, &Material::default());
    }

    #[test]
    fn assigning_material() {
        let mut s = TestShape::new();
        let mut m = Material::default();
        m.ambient = 1.0;
        s.set_material(Material::clone(&m));
        assert_eq!(s.material(), &m);
    }

    #[test]
    fn intersecting_scaled_ray() {
        let r = Ray::new(point(0, 0, -5), vector(0, 0, 1));
        let mut s = TestShape::new();
        s.set_transform(scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(&r);

        assert_eq!(
            s.saved_ray.lock().unwrap().as_ref().unwrap().origin(),
            point(0, 0, -2.5)
        );
        assert_eq!(
            s.saved_ray.lock().unwrap().as_ref().unwrap().direction(),
            vector(0, 0, 0.5)
        );
    }

    #[test]
    fn intersecting_translated_ray() {
        let r = Ray::new(point(0, 0, -5), vector(0, 0, 1));
        let mut s = TestShape::new();
        s.set_transform(translation(5.0, 0.0, 0.0));
        let xs = s.intersect(&r);

        assert_eq!(
            s.saved_ray.lock().unwrap().as_ref().unwrap().origin(),
            point(-5, 0, -5)
        );
        assert_eq!(
            s.saved_ray.lock().unwrap().as_ref().unwrap().direction(),
            vector(0, 0, 1)
        );
    }

    #[test]
    fn normal_on_translated() {
        let mut s = TestShape::new();
        s.set_transform(translation(0.0, 1.0, 0.0));
        let n = s.normal_at(point(0, 1.70711, -0.70711));
        assert_eq!(n, vector(0, 0.70711, -0.70711));
    }

    #[test]
    fn normal_on_transformed() {
        let mut s = TestShape::new();
        let m = TransformationBuilder::create()
            .rotation_z(PI / 5.0)
            .scaling(1.0, 0.5, 1.0)
            .build();
        s.set_transform(m);
        let n = s.normal_at(point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0));
        assert_eq!(n, vector(0, 0.97014, -0.24254));
    }
}
