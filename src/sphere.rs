use crate::{
    matrix::helpers::Mat4,
    ray::{Intersection, Ray},
    tuples::helpers::point,
};

#[derive(PartialEq, Debug)]
pub struct Sphere {
    transform: Mat4,
}

impl Sphere {
    pub fn new() -> Sphere {
        Self {
            transform: Mat4::identity(),
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
}

#[cfg(test)]
mod tests {
    use crate::{
        matrix::helpers::Mat4,
        ray::Ray,
        transformation::{scaling, translation},
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
}
