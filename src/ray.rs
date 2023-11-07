use crate::{
    matrix::helpers::Mat4,
    shapes::{Shape, Sphere},
    tuples::{Tuple, EPSILON},
};

#[derive(Debug, PartialEq)]
pub struct Ray {
    origin: Tuple,
    direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray { origin, direction }
    }
    pub fn origin(&self) -> Tuple {
        self.origin
    }
    pub fn direction(&self) -> Tuple {
        self.direction
    }
    pub fn at(&self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }
    pub fn transform(&self, transform: &Mat4) -> Ray {
        Ray::new(
            transform.clone() * self.origin(),
            transform.clone() * self.direction(),
        )
    }
}

#[derive(Debug, Clone)]
pub struct Intersection<'a> {
    object: &'a dyn Shape,
    t: f64,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a dyn Shape) -> Self {
        Intersection { object, t }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &'a dyn Shape {
        self.object
    }

    pub fn prepare_comps(&self, ray: &Ray) -> Computations {
        let point = ray.at(self.t);
        let mut normalv = self.object.normal_at(point);
        let inside = normalv.dot(&-ray.direction()) < 0.0;
        normalv *= if inside { -1.0 } else { 1.0 };

        Computations {
            t: self.t,
            object: self.object,
            point,
            eyev: -ray.direction(),
            normalv,
            inside,
            over_point: point + normalv * EPSILON,
        }
    }
}

pub struct Computations<'a> {
    pub t: f64,
    pub object: &'a dyn Shape,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
}

pub trait Intersections {
    fn hit(&self) -> Option<Intersection>;
}

impl Intersections for Vec<Intersection<'_>> {
    fn hit(&self) -> Option<Intersection> {
        self.iter()
            .filter(|f| f.t() >= 0.0)
            .min_by(|a, b| a.t().partial_cmp(&b.t()).unwrap())
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ray::Intersections,
        shapes::{Shape, Sphere},
        transformation::{scaling, translation},
        tuples::{
            helpers::{point, vector},
            FEquals, EPSILON,
        },
    };

    use super::{Intersection, Ray};

    #[test]
    fn creating_a_ray() {
        let origin = point(1, 2, 3);
        let direction = vector(4, 5, 6);

        let r = Ray::new(origin, direction);

        assert_eq!(r.origin(), origin);
        assert_eq!(r.direction(), direction);
    }

    #[test]
    fn computing_at() {
        let r = Ray::new(point(2, 3, 4), vector(1, 0, 0));

        assert_eq!(r.at(0.0), point(2, 3, 4));
        assert_eq!(r.at(1.0), point(3, 3, 4));
        assert_eq!(r.at(-1.0), point(1, 3, 4));
        assert_eq!(r.at(2.5), point(4.5, 3, 4));
    }

    #[test]
    fn intersection_encapsulates() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);

        assert!(i.t().eps_eq(3.5));
        // assert_eq!(i.object(), &s); //TODO:
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);

        let xs = vec![i1, i2];

        assert_eq!(xs.len(), 2);
        assert!(xs[0].t().eps_eq(1.0));
        assert!(xs[1].t().eps_eq(2.0));
    }

    #[test]
    fn hit_all_positive_t() {
        let s = Sphere::new();

        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);

        let xs = vec![i2, i1];
        let i = xs.hit();

        // assert_eq!(i, Some(i1)); //TODO
    }

    #[test]
    fn hit_some_negative_t() {
        let s = Sphere::new();

        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);

        let xs = vec![i2, i1];
        let i = xs.hit();

        // assert_eq!(i, Some(i2));//TODO
    }

    #[test]
    fn hit_all_negative_t() {
        let s = Sphere::new();

        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);

        let xs = vec![i2, i1];
        let i = xs.hit();

        // assert_eq!(i, None);//TODO
    }

    #[test]
    fn hit_lowest_t() {
        let s = Sphere::new();

        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);

        let xs = vec![i1, i2, i3, i4];
        let i = xs.hit();

        // assert_eq!(i, Some(i4)); //TODO:
    }

    #[test]
    fn translating_a_ray() {
        let r = Ray::new(point(1, 2, 3), vector(0, 1, 0));
        let m = translation(3.0, 4.0, 5.0);
        let r2 = r.transform(&m);
        assert_eq!(r2.origin(), point(4, 6, 8));
        assert_eq!(r2.direction(), vector(0, 1, 0));
    }

    #[test]
    fn scaling_a_ray() {
        let r = Ray::new(point(1, 2, 3), vector(0, 1, 0));
        let m = scaling(2.0, 3.0, 4.0);
        let r2 = r.transform(&m);
        assert_eq!(r2.origin(), point(2, 6, 12));
        assert_eq!(r2.direction(), vector(0, 3, 0));
    }

    #[test]
    fn precomputing_intersections() {
        let r = Ray::new(point(0, 0, -5), vector(0, 0, 1));
        let shape = Sphere::new();
        let i = Intersection::new(4.0, &shape);
        let comps = i.prepare_comps(&r);

        assert!(comps.t.eps_eq(i.t));
        // assert_eq!(comps.object, i.object); //TODO
        assert_eq!(comps.point, point(0, 0, -1));
        assert_eq!(comps.eyev, vector(0, 0, -1));
        assert_eq!(comps.normalv, vector(0, 0, -1));
    }

    #[test]
    fn hit_on_outside() {
        let r = Ray::new(point(0, 0, -5), vector(0, 0, 1));
        let shape = Sphere::new();
        let i = Intersection::new(4.0, &shape);
        let comps = i.prepare_comps(&r);
        assert_eq!(comps.inside, false);
    }

    #[test]
    fn hit_on_inside() {
        let r = Ray::new(point(0, 0, 0), vector(0, 0, 1));
        let shape = Sphere::new();
        let i = Intersection::new(1.0, &shape);
        let comps = i.prepare_comps(&r);
        assert_eq!(comps.inside, true);
        assert_eq!(comps.point, point(0, 0, 1));
        assert_eq!(comps.eyev, vector(0, 0, -1));
        assert_eq!(comps.normalv, vector(0, 0, -1));
    }

    #[test]
    fn hit_offsets_point() {
        let r = Ray::new(point(0, 0, -5), vector(0, 0, 1));
        let mut shape = Sphere::new();
        shape.set_transform(translation(0.0, 0.0, 1.0));
        let i = Intersection::new(5.0, &shape);

        let comps = i.prepare_comps(&r);
        assert!(comps.over_point.z() < -EPSILON / 2.0);
        assert!(comps.point.z() > comps.over_point.z());
    }
}
