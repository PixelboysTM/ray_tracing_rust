use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, MulAssign, Neg, Sub},
    rc::Rc,
};

use crate::patterns::Solid;

#[derive(Clone, Copy)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Tuple {
    pub const fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { x, y, z, w }
    }
    pub const fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple::new(x, y, z, 1.0)
    }
    pub const fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple::new(x, y, z, 0.0)
    }
    pub const fn color(r: f64, g: f64, b: f64) -> Tuple {
        Tuple::new(r, g, b, 0.0)
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
    pub fn r(&self) -> f64 {
        self.x
    }
    pub fn g(&self) -> f64 {
        self.y
    }
    pub fn b(&self) -> f64 {
        self.z
    }

    pub fn is_point(&self) -> bool {
        self.w.eps_eq(1.0)
    }
    pub fn is_vector(&self) -> bool {
        self.w.eps_eq(0.0)
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
        assert!(self.w.eps_eq(0.0));
        assert!(other.w.eps_eq(0.0));

        Tuple::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
    pub fn reflect(&self, normal: Tuple) -> Tuple {
        *self - normal * 2.0 * self.dot(&normal)
    }

    pub fn solid(self) -> Rc<Solid> {
        Rc::new(Solid::new(self))
    }
}

impl Debug for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{},{})", self.x(), self.y(), self.z(), self.w())
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        self.x.eps_eq(other.x)
            && self.y.eps_eq(other.y)
            && self.z.eps_eq(other.z)
            && self.w.eps_eq(other.w)
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
        Tuple::new(0.0, 0.0, 0.0, 0.0) - self
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Tuple::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl MulAssign<f64> for Tuple {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Mul for Tuple {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert!(!(self.w().eps_eq(1.0)));
        assert!(!(rhs.w().eps_eq(1.0)));

        Tuple::color(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

pub const EPSILON: f64 = 0.00001;
pub trait FEquals {
    type Rhs;
    fn eps_eq(&self, rhs: Self::Rhs) -> bool;
}
impl FEquals for f64 {
    fn eps_eq(&self, rhs: Self::Rhs) -> bool {
        (self - rhs).abs() < EPSILON
    }

    type Rhs = f64;
}

pub mod helpers {
    use crate::{matrix::helpers::Mat4, patterns::Pattern};

    use super::Tuple;

    pub fn tuple<F1, F2, F3, F4>(x: F1, y: F2, z: F3, w: F4) -> Tuple
    where
        F1: Into<f64>,
        F2: Into<f64>,
        F3: Into<f64>,
        F4: Into<f64>,
    {
        Tuple::new(x.into(), y.into(), z.into(), w.into())
    }

    pub fn point<F1, F2, F3>(x: F1, y: F2, z: F3) -> Tuple
    where
        F1: Into<f64>,
        F2: Into<f64>,
        F3: Into<f64>,
    {
        Tuple::point(x.into(), y.into(), z.into())
    }

    pub fn vector<F1, F2, F3>(x: F1, y: F2, z: F3) -> Tuple
    where
        F1: Into<f64>,
        F2: Into<f64>,
        F3: Into<f64>,
    {
        Tuple::vector(x.into(), y.into(), z.into())
    }

    pub fn color<C1, C2, C3>(r: C1, g: C2, b: C3) -> Tuple
    where
        C1: Into<f64>,
        C2: Into<f64>,
        C3: Into<f64>,
    {
        Tuple::color(r.into(), g.into(), b.into())
    }

    pub mod colors {
        use crate::tuples::Tuple;

        pub const fn red() -> Tuple {
            Tuple::color(1.0, 0.0, 0.0)
        }

        pub const fn green() -> Tuple {
            Tuple::color(0.0, 1.0, 0.0)
        }

        pub const fn blue() -> Tuple {
            Tuple::color(0.0, 0.0, 1.0)
        }
        pub const fn white() -> Tuple {
            Tuple::color(1.0, 1.0, 1.0)
        }
        pub const fn black() -> Tuple {
            Tuple::color(0.0, 0.0, 0.0)
        }
    }

    pub mod points {
        use crate::tuples::Tuple;

        pub const fn zero() -> Tuple {
            Tuple::point(0.0, 0.0, 0.0)
        }

        pub const fn one() -> Tuple {
            Tuple::point(1.0, 1.0, 1.0)
        }
    }

    pub mod vector {
        use crate::tuples::Tuple;

        pub const fn zero() -> Tuple {
            Tuple::vector(0.0, 0.0, 0.0)
        }

        pub const fn one() -> Tuple {
            Tuple::vector(1.0, 1.0, 1.0)
        }

        pub const fn up() -> Tuple {
            Tuple::vector(0.0, 1.0, 0.0)
        }
    }
}

#[cfg(test)]
mod tests {
    pub use super::helpers::*;
    use super::*;

    #[test]
    fn tuple_point() {
        let a = tuple(4.3, -4.2, 3.1, 1.0);
        assert!(a.x.eps_eq(4.3));
        assert!(a.y.eps_eq(-4.2));
        assert!(a.z.eps_eq(3.1));
        assert!(a.w.eps_eq(1.0));
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn tuple_vector() {
        let a = tuple(4.3, -4.2, 3.1, 0.0);
        assert!(a.x.eps_eq(4.3));
        assert!(a.y.eps_eq(-4.2));
        assert!(a.z.eps_eq(3.1));
        assert!(a.w.eps_eq(0.0));
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn new_point() {
        let p = point(4.0, -4.0, 3.0);
        assert_eq!(p, tuple(4.0, -4.0, 3.0, 1.0))
    }

    #[test]
    fn new_vector() {
        let p = vector(4.0, -4.0, 3.0);
        assert_eq!(p, tuple(4.0, -4.0, 3.0, 0.0))
    }

    #[test]
    fn adding_two_tuples() {
        let a1 = tuple(3, -2, 5, 1);
        let a2 = tuple(-2, 3, 1, 0);
        assert_eq!(a1 + a2, tuple(1, 1, 6, 1));
    }

    #[test]
    fn subtractring_two_points() {
        let p1 = point(3, 2, 1);
        let p2 = point(5, 6, 7);
        assert_eq!(p1 - p2, vector(-2, -4, -6));
    }

    #[test]
    fn subtractring_vector_from_point() {
        let p = point(3, 2, 1);
        let v = vector(5, 6, 7);
        assert_eq!(p - v, point(-2, -4, -6))
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = vector(3, 2, 1);
        let v2 = vector(5, 6, 7);
        assert_eq!(v1 - v2, vector(-2, -4, -6));
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
        assert!(norm.magnitude().eps_eq(1.0));
    }

    #[test]
    fn dot_of_tuples() {
        let a = vector(1, 2, 3);
        let b = vector(2, 3, 4);

        assert!(a.dot(&b).eps_eq(20.0));
    }

    #[test]
    fn cross_of_vectors() {
        let a = vector(1, 2, 3);
        let b = vector(2, 3, 4);
        assert_eq!(a.cross(&b), vector(-1, 2, -1));
        assert_eq!(b.cross(&a), vector(1, -2, 1));
    }

    #[test]
    fn colors_are_tuples() {
        let c = color(-0.5, 0.4, 1.7);

        assert!(c.r().eps_eq(-0.5));
        assert!(c.g().eps_eq(0.4));
        assert!(c.b().eps_eq(1.7));
    }

    #[test]
    fn adding_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, color(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtracting_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        assert_eq!(c1 - c2, color(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiply_color_scalar() {
        let c = color(0.2, 0.3, 0.4);
        assert_eq!(c * 2.0, color(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiplying_colors() {
        let c1 = color(1, 0.2, 0.4);
        let c2 = color(0.9, 1, 0.1);
        assert_eq!(c1 * c2, color(0.9, 0.2, 0.04));
    }

    #[test]
    fn reflect_45() {
        let v = vector(1, -1, 0);
        let n = vector(0, 1, 0);
        let r = v.reflect(n);
        assert_eq!(r, vector(1, 1, 0));
    }

    #[test]
    fn reflect_slanted() {
        let v = vector(0, -1, 0);
        let n = vector(2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0);
        let r = v.reflect(n);
        assert_eq!(r, vector(1, 0, 0));
    }
}
