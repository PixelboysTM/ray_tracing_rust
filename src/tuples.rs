use std::{
    fmt::{write, Debug},
    ops::{Add, Div, Mul, Neg, Sub},
};

#[derive(Clone, Copy)]
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

    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }
    pub fn w(&self) -> f64 {
        self.w
    }

    pub fn is_point(&self) -> bool {
        self.w.eps_eq(&1.0)
    }
    pub fn is_vector(&self) -> bool {
        self.w.eps_eq(&0.0)
    }
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }
    pub fn normalized(&self) -> Tuple {
        *self * (1.0 / self.magnitude())
    }
    pub fn dot(&self, b: &Self) -> f64 {
        self.x * b.x + self.y * b.y + self.z * b.z + self.w * b.w
    }
    pub fn cross(&self, other: &Self) -> Tuple {
        assert!(self.w.eps_eq(&0.0));
        assert!(other.w.eps_eq(&0.0));

        Tuple::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl Debug for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{},{})", self.x(), self.y(), self.z(), self.w())
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

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Tuple::new(0, 0, 0, 0) - self
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        helpers::tuple(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

const EPSILON: f64 = 0.00001;
trait FEquals {
    fn eps_eq(&self, rhs: &Self) -> bool;
}
impl FEquals for f64 {
    fn eps_eq(&self, rhs: &Self) -> bool {
        (self - rhs).abs() < EPSILON
    }
}

pub mod helpers {
    use super::Tuple;

    pub fn tuple<F1, F2, F3, F4>(x: F1, y: F2, z: F3, w: F4) -> Tuple
    where
        F1: Into<f64>,
        F2: Into<f64>,
        F3: Into<f64>,
        F4: Into<f64>,
    {
        Tuple::new(x, y, z, w)
    }

    pub fn point<F1, F2, F3>(x: F1, y: F2, z: F3) -> Tuple
    where
        F1: Into<f64>,
        F2: Into<f64>,
        F3: Into<f64>,
    {
        Tuple::point(x, y, z)
    }

    pub fn vector<F1, F2, F3>(x: F1, y: F2, z: F3) -> Tuple
    where
        F1: Into<f64>,
        F2: Into<f64>,
        F3: Into<f64>,
    {
        Tuple::vector(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    pub use super::helpers::*;
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

    #[test]
    fn subtracting_vector_from_zero() {
        let zero = vector(0, 0, 0);
        let v = vector(1, -2, 3);
        assert_eq!(zero - v, vector(-1, 2, -3));
    }

    #[test]
    fn negating_a_tuple() {
        let a = tuple(1, -2, 3, -4);
        assert_eq!(-a, tuple(-1, 2, -3, 4));
    }

    #[test]
    fn mul_tuple_scalar() {
        let a = tuple(1, -2, 3, -4);
        assert_eq!(a * 3.5, tuple(3.5, -7, 10.5, -14));
    }

    #[test]
    fn mul_tuple_fraction() {
        let a = tuple(1, -2, 3, -4);
        assert_eq!(a * 0.5, tuple(0.5, -1, 1.5, -2));
    }

    #[test]
    fn div_tuple_scalar() {
        let a = tuple(1, -2, 3, -4);
        assert_eq!(a / 2.0, tuple(0.5, -1, 1.5, -2));
    }

    #[test]
    fn computing_magnitude() {
        let v = vector(1, 0, 0);
        assert_eq!(v.magnitude(), 1.0);
        let v = vector(0, 1, 0);
        assert_eq!(v.magnitude(), 1.0);
        let v = vector(0, 0, 1);
        assert_eq!(v.magnitude(), 1.0);

        let v = vector(1, 2, 3);
        assert_eq!(v.magnitude(), 14.0_f64.sqrt());
        let v = vector(-1, -2, -3);
        assert_eq!(v.magnitude(), 14.0_f64.sqrt());
    }

    #[test]
    fn normalization_vector() {
        let v = vector(4, 0, 0);
        assert_eq!(v.normalized(), vector(1, 0, 0));

        let v = vector(1, 2, 3);
        assert_eq!(
            v.normalized(),
            vector(
                1.0 / 14_f64.sqrt(),
                2.0 / 14_f64.sqrt(),
                3.0 / 14_f64.sqrt()
            )
        );

        let norm = v.normalized();
        assert!(norm.magnitude().eps_eq(&1.0));
    }

    #[test]
    fn dot_of_tuples() {
        let a = vector(1, 2, 3);
        let b = vector(2, 3, 4);

        assert!(a.dot(&b).eps_eq(&20.0));
    }

    #[test]
    fn cross_of_vectors() {
        let a = vector(1, 2, 3);
        let b = vector(2, 3, 4);
        assert_eq!(a.cross(&b), vector(-1, 2, -1));
        assert_eq!(b.cross(&a), vector(1, -2, 1));
    }
}
