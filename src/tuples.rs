use std::ops::{Add, AddAssign, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Tuple {
    pub fn new<F1, F2, F3, F4>(x: F1, y: F2, z: F3, w: F4) -> Tuple
    where
        F1: Into<f64>,
        F2: Into<f64>,
        F3: Into<f64>,
        F4: Into<f64>,
    {
        Tuple {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: w.into(),
        }
    }
    pub fn point<F1, F2, F3>(x: F1, y: F2, z: F3) -> Tuple
    where
        F1: Into<f64>,
        F2: Into<f64>,
        F3: Into<f64>,
    {
        Tuple::new(x, y, z, 1.0)
    }
    pub fn vector<F1, F2, F3>(x: F1, y: F2, z: F3) -> Tuple
    where
        F1: Into<f64>,
        F2: Into<f64>,
        F3: Into<f64>,
    {
        Tuple::new(x, y, z, 0.0)
    }

    pub fn is_point(&self) -> bool {
        self.w.eps_eq(&1.0)
    }
    pub fn is_vector(&self) -> bool {
        self.w.eps_eq(&0.0)
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        self.x.eps_eq(&other.x)
            && self.y.eps_eq(&other.y)
            && self.z.eps_eq(&other.z)
            && self.w.eps_eq(&other.w)
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Tuple::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Tuple::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

trait FEquals {
    fn eps_eq(&self, rhs: &Self) -> bool;
}
impl FEquals for f64 {
    fn eps_eq(&self, rhs: &Self) -> bool {
        (self - rhs).abs() < f64::EPSILON
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert!(a.x.eps_eq(&4.3));
        assert!(a.y.eps_eq(&-4.2));
        assert!(a.z.eps_eq(&3.1));
        assert!(a.w.eps_eq(&1.0));
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn tuple_vector() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert!(a.x.eps_eq(&4.3));
        assert!(a.y.eps_eq(&-4.2));
        assert!(a.z.eps_eq(&3.1));
        assert!(a.w.eps_eq(&0.0));
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn new_point() {
        let p = Tuple::point(4.0, -4.0, 3.0);
        assert_eq!(p, Tuple::new(4.0, -4.0, 3.0, 1.0))
    }

    #[test]
    fn new_vector() {
        let p = Tuple::vector(4.0, -4.0, 3.0);
        assert_eq!(p, Tuple::new(4.0, -4.0, 3.0, 0.0))
    }

    #[test]
    fn adding_two_tuples() {
        let a1 = Tuple::new(3, -2, 5, 1);
        let a2 = Tuple::new(-2, 3, 1, 0);
        assert_eq!(a1 + a2, Tuple::new(1, 1, 6, 1));
    }

    #[test]
    fn subtractring_two_points() {
        let p1 = Tuple::point(3, 2, 1);
        let p2 = Tuple::point(5, 6, 7);
        assert_eq!(p1 - p2, Tuple::vector(-2, -4, -6));
    }

    #[test]
    fn subtractring_vector_from_point() {
        let p = Tuple::point(3, 2, 1);
        let v = Tuple::vector(5, 6, 7);
        assert_eq!(p - v, Tuple::point(-2, -4, -6))
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = Tuple::vector(3, 2, 1);
        let v2 = Tuple::vector(5, 6, 7);
        assert_eq!(v1 - v2, Tuple::vector(-2, -4, -6));
    }
}
