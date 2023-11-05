use crate::{
    material::Material,
    matrix::helpers::Mat4,
    ray::{Intersection, Ray},
    tuples::{
        helpers::{point, points},
        Tuple,
    },
};

#[derive(PartialEq, Debug)]
pub struct Sphere {
    transform: Mat4,
    material: Material,
}

impl Sphere {
    pub fn new() -> Sphere {
        Self {
            transform: Mat4::identity(),
            material: Material::default(),
        }
    }
    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let ray2 = ray.transform(&self.transform().inverse());

        let sphere_to_ray = ray2.origin() - point(0, 0, 0);

        let a = ray2.direction().dot(&ray2.direction());
        let b = 2.0 * ray2.direction().dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return Vec::new();
        }

        let disc_sqrt = discriminant.sqrt();

        let t1 = (-b - disc_sqrt) / (2.0 * a);
        let t2 = (-b + disc_sqrt) / (2.0 * a);

        return vec![Intersection::new(t1, self), Intersection::new(t2, self)];
    }

    pub fn transform(&self) -> &Mat4 {
        &self.transform
    }
    pub fn set_transform(&mut self, new_transform: Mat4) {
        self.transform = new_transform;
    }
    pub fn normal_at(&self, p: Tuple) -> Tuple {
        let object_point = self.transform.inverse() * p;
        let object_normal = object_point - points::zero();
        let world_normal = self.transform.inverse().transpose() * object_normal;

        (Tuple::vector(world_normal.x(), world_normal.y(), world_normal.z())).normalized()
    }
    pub fn material(&self) -> &Material {
        &self.material
    }
    pub fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }
    pub fn set_material(&mut self, new_material: Material) {
        self.material = new_material;
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        material::Material,
        matrix::helpers::Mat4,
        ray::Ray,
        transformation::{rotation_z, scaling, translation, PI},
        tuples::{
            helpers::{point, vector},
            FEquals,
        },
    };

    use super::Sphere;

    #[test]
    fn ray_sphere_two_points() {
        let r = Ray::new(point(0, 0, -5), vector(0, 0, 1));
        let s = Sphere::new();

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert!(xs[0].t().eps_eq(4.0));
        assert!(xs[1].t().eps_eq(6.0));
    }

    #[test]
    fn ray_sphere_tangent() {
        let r = Ray::new(point(0, 1, -5), vector(0, 0, 1));
        let s = Sphere::new();

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert!(xs[0].t().eps_eq(5.0));
        assert!(xs[1].t().eps_eq(5.0));
    }

    #[test]
    fn ray_sphere_miss() {
        let r = Ray::new(point(0, 2, -5), vector(0, 0, 1));
        let s = Sphere::new();

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_in_sphere_two_points() {
        let r = Ray::new(point(0, 0, 0), vector(0, 0, 1));
        let s = Sphere::new();

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert!(xs[0].t().eps_eq(-1.0));
        assert!(xs[1].t().eps_eq(1.0));
    }

    #[test]
    fn ray_infornt_sphere_two_points() {
        let r = Ray::new(point(0, 0, 5), vector(0, 0, 1));
        let s = Sphere::new();

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert!(xs[0].t().eps_eq(-6.0));
        assert!(xs[1].t().eps_eq(-4.0));
    }

    #[test]
    fn intersect_sets_object() {
        let r = Ray::new(point(0, 0, -5), vector(0, 0, 1));

        let s = Sphere::new();

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object(), &s);
        assert_eq!(xs[1].object(), &s);
    }

    #[test]
    fn sphere_transform() {
        let s = Sphere::new();

        assert_eq!(s.transform(), &Mat4::identity());
    }

    #[test]
    fn changing_sphere_transform() {
        let mut s = Sphere::new();
        let t = translation(2.0, 3.0, 4.0);

        s.set_transform(t.clone());

        assert_eq!(s.transform(), &t);
    }

    #[test]
    fn intersecting_scaled_ray() {
        let r = Ray::new(point(0, 0, -5), vector(0, 0, 1));
        let mut s = Sphere::new();
        s.set_transform(scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert!(xs[0].t().eps_eq(3.0));
        assert!(xs[1].t().eps_eq(7.0));
    }

    #[test]
    fn intersecting_translated_ray() {
        let r = Ray::new(point(0, 0, -5), vector(0, 0, 1));
        let mut s = Sphere::new();
        s.set_transform(translation(5.0, 0.0, 0.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn normal_sphere_x() {
        let s = Sphere::new();
        let n = s.normal_at(point(1, 0, 0));
        assert_eq!(n, vector(1, 0, 0));
    }

    #[test]
    fn normal_sphere_y() {
        let s = Sphere::new();
        let n = s.normal_at(point(0, 1, 0));
        assert_eq!(n, vector(0, 1, 0));
    }

    #[test]
    fn normal_sphere_z() {
        let s = Sphere::new();
        let n = s.normal_at(point(0, 0, 1));
        assert_eq!(n, vector(0, 0, 1));
    }

    #[test]
    fn normal_sphere_nonaxial() {
        let t: f64 = 3.0_f64.sqrt() / 3.0;

        let s = Sphere::new();
        let n = s.normal_at(point(t, t, t));
        assert_eq!(n, vector(t, t, t));
    }

    #[test]
    fn normal_is_normalized() {
        let t: f64 = 3.0_f64.sqrt() / 3.0;
        let s = Sphere::new();
        let n = s.normal_at(point(t, t, t));
        assert_eq!(n, n.normalized())
    }

    #[test]
    fn normal_on_translated() {
        let mut s = Sphere::new();
        s.set_transform(translation(0.0, 1.0, 0.0));
        let n = s.normal_at(point(0, 1.70711, -0.70711));
        assert_eq!(n, vector(0, 0.70711, -0.70711));
    }

    #[test]
    fn normal_on_transformed() {
        let mut s = Sphere::new();
        s.set_transform(scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0));
        let n = s.normal_at(point(0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0));
        assert_eq!(n, vector(0, 0.97014, -0.24254));
    }

    #[test]
    fn sphere_default_material() {
        let s = Sphere::new();

        let m = s.material();

        assert_eq!(m, &Material::default());
    }

    #[test]
    fn sphere_modified_material() {
        let mut s = Sphere::new();

        let mut m = Material::default();

        m.ambient = 1.0;
        s.set_material(m.clone());
        assert_eq!(s.material(), &m);
    }
}
