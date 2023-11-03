use std::{
    fmt::Debug,
    ops::{Index, IndexMut, Mul},
};

use crate::tuples::{FEquals, Tuple};

#[derive(Clone)]
pub struct Matrix4x4 {
    m: [[f64; 4]; 4],
}

impl Matrix4x4 {
    pub fn new(
        m00: f64,
        m01: f64,
        m02: f64,
        m03: f64,
        m10: f64,
        m11: f64,
        m12: f64,
        m13: f64,
        m20: f64,
        m21: f64,
        m22: f64,
        m23: f64,
        m30: f64,
        m31: f64,
        m32: f64,
        m33: f64,
    ) -> Matrix4x4 {
        Matrix4x4 {
            m: [
                [m00, m10, m20, m30],
                [m01, m11, m21, m31],
                [m02, m12, m22, m32],
                [m03, m13, m23, m33],
            ],
        }
    }
}

impl Index<(usize, usize)> for Matrix4x4 {
    type Output = f64;

    fn index(&self, (r, c): (usize, usize)) -> &Self::Output {
        &self.m[c][r]
    }
}

impl IndexMut<(usize, usize)> for Matrix4x4 {
    fn index_mut(&mut self, (r, c): (usize, usize)) -> &mut Self::Output {
        &mut self.m[c][r]
    }
}

#[derive(Clone)]
pub struct Matrix3x3 {
    m: [[f64; 3]; 3],
}

impl Matrix3x3 {
    pub fn new(
        m00: f64,
        m01: f64,
        m02: f64,
        m10: f64,
        m11: f64,
        m12: f64,
        m20: f64,
        m21: f64,
        m22: f64,
    ) -> Matrix3x3 {
        Matrix3x3 {
            m: [[m00, m10, m20], [m01, m11, m21], [m02, m12, m22]],
        }
    }
}

impl Index<(usize, usize)> for Matrix3x3 {
    type Output = f64;

    fn index(&self, (r, c): (usize, usize)) -> &Self::Output {
        &self.m[c][r]
    }
}

impl IndexMut<(usize, usize)> for Matrix3x3 {
    fn index_mut(&mut self, (r, c): (usize, usize)) -> &mut Self::Output {
        &mut self.m[c][r]
    }
}

#[derive(Clone)]
pub struct Matrix2x2 {
    m: [[f64; 2]; 2],
}

impl Matrix2x2 {
    pub fn new(m00: f64, m01: f64, m10: f64, m11: f64) -> Matrix2x2 {
        Matrix2x2 {
            m: [[m00, m10], [m01, m11]],
        }
    }
}

impl Index<(usize, usize)> for Matrix2x2 {
    type Output = f64;

    fn index(&self, (r, c): (usize, usize)) -> &Self::Output {
        &self.m[c][r]
    }
}

impl IndexMut<(usize, usize)> for Matrix2x2 {
    fn index_mut(&mut self, (r, c): (usize, usize)) -> &mut Self::Output {
        &mut self.m[c][r]
    }
}

impl PartialEq for Matrix4x4 {
    fn eq(&self, other: &Self) -> bool {
        for x in 0..4 {
            for y in 0..4 {
                if !self[(x, y)].eps_eq(other[(x, y)]) {
                    return false;
                }
            }
        }

        true
    }
}

impl PartialEq for Matrix3x3 {
    fn eq(&self, other: &Self) -> bool {
        for x in 0..3 {
            for y in 0..3 {
                if !self[(x, y)].eps_eq(other[(x, y)]) {
                    return false;
                }
            }
        }

        true
    }
}

impl PartialEq for Matrix2x2 {
    fn eq(&self, other: &Self) -> bool {
        for x in 0..2 {
            for y in 0..2 {
                if !self[(x, y)].eps_eq(other[(x, y)]) {
                    return false;
                }
            }
        }

        true
    }
}

impl Mul for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut m = Matrix4x4::zero();

        for row in 0..4 {
            for col in 0..4 {
                m[(row, col)] = self[(row, 0)] * rhs[(0, col)]
                    + self[(row, 1)] * rhs[(1, col)]
                    + self[(row, 2)] * rhs[(2, col)]
                    + self[(row, 3)] * rhs[(3, col)];
            }
        }

        m
    }
}

impl Mul<Tuple> for Matrix4x4 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        Tuple::new(
            self[(0, 0)] * rhs.x()
                + self[(0, 1)] * rhs.y()
                + self[(0, 2)] * rhs.z()
                + self[(0, 3)] * rhs.w(),
            self[(1, 0)] * rhs.x()
                + self[(1, 1)] * rhs.y()
                + self[(1, 2)] * rhs.z()
                + self[(1, 3)] * rhs.w(),
            self[(2, 0)] * rhs.x()
                + self[(2, 1)] * rhs.y()
                + self[(2, 2)] * rhs.z()
                + self[(2, 3)] * rhs.w(),
            self[(3, 0)] * rhs.x()
                + self[(3, 1)] * rhs.y()
                + self[(3, 2)] * rhs.z()
                + self[(3, 3)] * rhs.w(),
        )
    }
}

impl Mul for Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut m = Matrix3x3::zero();

        for row in 0..3 {
            for col in 0..3 {
                m[(row, col)] = self[(row, 0)] * rhs[(0, col)]
                    + self[(row, 1)] * rhs[(1, col)]
                    + self[(row, 2)] * rhs[(2, col)]
            }
        }

        m
    }
}

impl Mul for Matrix2x2 {
    type Output = Matrix2x2;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut m = Matrix2x2::zero();

        for row in 0..4 {
            for col in 0..4 {
                m[(row, col)] = self[(row, 0)] * rhs[(0, col)] + self[(row, 1)] * rhs[(1, col)]
            }
        }

        m
    }
}

impl Matrix4x4 {
    pub fn transpose(&self) -> Matrix4x4 {
        Matrix4x4::new(
            self[(0, 0)],
            self[(1, 0)],
            self[(2, 0)],
            self[(3, 0)],
            self[(0, 1)],
            self[(1, 1)],
            self[(2, 1)],
            self[(3, 1)],
            self[(0, 2)],
            self[(1, 2)],
            self[(2, 2)],
            self[(3, 2)],
            self[(0, 3)],
            self[(1, 3)],
            self[(2, 3)],
            self[(3, 3)],
        )
    }
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix3x3 {
        assert!(row < 4);
        assert!(col < 4);

        fn index(i: usize) -> (usize, usize, usize) {
            match i {
                0 => (1, 2, 3),
                1 => (0, 2, 3),
                2 => (0, 1, 3),
                3 => (0, 1, 2),
                _ => panic!("WTF"),
            }
        }

        let (x0, x1, x2) = index(col);
        let (y0, y1, y2) = index(row);

        Matrix3x3::new(
            self[(y0, x0)],
            self[(y0, x1)],
            self[(y0, x2)],
            self[(y1, x0)],
            self[(y1, x1)],
            self[(y1, x2)],
            self[(y2, x0)],
            self[(y2, x1)],
            self[(y2, x2)],
        )
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        if (row + col) % 2 == 0 {
            self.minor(row, col)
        } else {
            -self.minor(row, col)
        }
    }

    pub fn determinant(&self) -> f64 {
        self[(0, 0)] * self.cofactor(0, 0)
            + self[(0, 1)] * self.cofactor(0, 1)
            + self[(0, 2)] * self.cofactor(0, 2)
            + self[(0, 3)] * self.cofactor(0, 3)
    }

    pub fn invertible(&self) -> bool {
        !self.determinant().eps_eq(0.0)
    }

    pub fn inverse(&self) -> Matrix4x4 {
        debug_assert!(self.invertible());

        let det = self.determinant();

        Matrix4x4::new(
            self.cofactor(0, 0) / det,
            self.cofactor(1, 0) / det,
            self.cofactor(2, 0) / det,
            self.cofactor(3, 0) / det,
            self.cofactor(0, 1) / det,
            self.cofactor(1, 1) / det,
            self.cofactor(2, 1) / det,
            self.cofactor(3, 1) / det,
            self.cofactor(0, 2) / det,
            self.cofactor(1, 2) / det,
            self.cofactor(2, 2) / det,
            self.cofactor(3, 2) / det,
            self.cofactor(0, 3) / det,
            self.cofactor(1, 3) / det,
            self.cofactor(2, 3) / det,
            self.cofactor(3, 3) / det,
        )
    }
}

impl Matrix3x3 {
    pub fn transpose(&self) -> Matrix3x3 {
        Matrix3x3::new(
            self[(0, 0)],
            self[(1, 0)],
            self[(2, 0)],
            self[(0, 1)],
            self[(1, 1)],
            self[(2, 1)],
            self[(0, 2)],
            self[(1, 2)],
            self[(2, 2)],
        )
    }
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix2x2 {
        assert!(row < 3);
        assert!(col < 3);

        fn index(i: usize) -> (usize, usize) {
            match i {
                0 => (1, 2),
                1 => (0, 2),
                2 => (0, 1),
                _ => panic!("WTF"),
            }
        }

        let (x0, x1) = index(col);
        let (y0, y1) = index(row);

        Matrix2x2::new(
            self[(y0, x0)],
            self[(y0, x1)],
            self[(y1, x0)],
            self[(y1, x1)],
        )
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        if (row + col) % 2 == 0 {
            self.minor(row, col)
        } else {
            -self.minor(row, col)
        }
    }

    pub fn determinant(&self) -> f64 {
        self[(0, 0)] * self.cofactor(0, 0)
            + self[(0, 1)] * self.cofactor(0, 1)
            + self[(0, 2)] * self.cofactor(0, 2)
    }
}

impl Matrix2x2 {
    pub fn transpose(&self) -> Matrix2x2 {
        Matrix2x2::new(self[(0, 0)], self[(1, 0)], self[(0, 1)], self[(1, 1)])
    }
    pub fn determinant(&self) -> f64 {
        self[(0, 0)] * self[(1, 1)] - self[(1, 0)] * self[(0, 1)]
    }
}

impl Debug for Matrix4x4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "|{:9.4}|{:9.4}|{:9.4}|{:9.4}|\n|{:9.4}|{:9.4}|{:9.4}|{:9.4}|\n|{:9.4}|{:9.4}|{:9.4}|{:9.4}|\n|{:9.4}|{:9.4}|{:9.4}|{:9.4}|\n", 
            self[(0,0)],
            self[(0,1)],
            self[(0,2)],
            self[(0,3)],
            self[(1,0)],
            self[(1,1)],
            self[(1,2)],
            self[(1,3)],
            self[(2,0)],
            self[(2,1)],
            self[(2,2)],
            self[(2,3)],
            self[(3,0)],
            self[(3,1)],
            self[(3,2)],
            self[(3,3)]
        )
    }
}

impl Debug for Matrix3x3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "|{:9.4}|{:9.4}|{:9.4}|\n|{:9.4}|{:9.4}|{:9.4}|\n|{:9.4}|{:9.4}|{:9.4}|\n",
            self[(0, 0)],
            self[(0, 1)],
            self[(0, 2)],
            self[(1, 0)],
            self[(1, 1)],
            self[(1, 2)],
            self[(2, 0)],
            self[(2, 1)],
            self[(2, 2)]
        )
    }
}

impl Debug for Matrix2x2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "|{:9.4}|{:9.4}|\n|{:9.4}|{:9.4}|\n",
            self[(0, 0)],
            self[(0, 1)],
            self[(1, 0)],
            self[(1, 1)],
        )
    }
}

pub mod helpers {
    use super::{Matrix2x2, Matrix3x3, Matrix4x4};

    pub type Mat4 = Matrix4x4;
    pub type Mat3 = Matrix3x3;
    pub type Mat2 = Matrix2x2;

    impl<M00, M01, M02, M03, M10, M11, M12, M13, M20, M21, M22, M23, M30, M31, M32, M33>
        Into<Matrix4x4>
        for (
            (M00, M10, M20, M30),
            (M01, M11, M21, M31),
            (M02, M12, M22, M32),
            (M03, M13, M23, M33),
        )
    where
        M00: Into<f64>,
        M01: Into<f64>,
        M02: Into<f64>,
        M03: Into<f64>,
        M10: Into<f64>,
        M11: Into<f64>,
        M12: Into<f64>,
        M13: Into<f64>,
        M20: Into<f64>,
        M21: Into<f64>,
        M22: Into<f64>,
        M23: Into<f64>,
        M30: Into<f64>,
        M31: Into<f64>,
        M32: Into<f64>,
        M33: Into<f64>,
    {
        fn into(self) -> Matrix4x4 {
            Matrix4x4::new(
                self.0 .0.into(),
                self.0 .1.into(),
                self.0 .2.into(),
                self.0 .3.into(),
                self.1 .0.into(),
                self.1 .1.into(),
                self.1 .2.into(),
                self.1 .3.into(),
                self.2 .0.into(),
                self.2 .1.into(),
                self.2 .2.into(),
                self.2 .3.into(),
                self.3 .0.into(),
                self.3 .1.into(),
                self.3 .2.into(),
                self.3 .3.into(),
            )
        }
    }

    impl<M00, M01, M02, M10, M11, M12, M20, M21, M22> Into<Matrix3x3>
        for ((M00, M10, M20), (M01, M11, M21), (M02, M12, M22))
    where
        M00: Into<f64>,
        M01: Into<f64>,
        M02: Into<f64>,
        M10: Into<f64>,
        M11: Into<f64>,
        M12: Into<f64>,
        M20: Into<f64>,
        M21: Into<f64>,
        M22: Into<f64>,
    {
        fn into(self) -> Matrix3x3 {
            Matrix3x3::new(
                self.0 .0.into(),
                self.0 .1.into(),
                self.0 .2.into(),
                self.1 .0.into(),
                self.1 .1.into(),
                self.1 .2.into(),
                self.2 .0.into(),
                self.2 .1.into(),
                self.2 .2.into(),
            )
        }
    }

    impl<M00, M01, M10, M11> Into<Matrix2x2> for ((M00, M10), (M01, M11))
    where
        M00: Into<f64>,
        M01: Into<f64>,
        M10: Into<f64>,
        M11: Into<f64>,
    {
        fn into(self) -> Matrix2x2 {
            Matrix2x2::new(
                self.0 .0.into(),
                self.0 .1.into(),
                self.1 .0.into(),
                self.1 .1.into(),
            )
        }
    }

    impl Mat4 {
        pub const fn zero() -> Mat4 {
            Mat4 { m: [[0.0; 4]; 4] }
        }
        pub const fn identity() -> Mat4 {
            Mat4 {
                m: [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ],
            }
        }
    }

    impl Mat3 {
        pub const fn zero() -> Mat3 {
            Mat3 { m: [[0.0; 3]; 3] }
        }
        pub const fn identity() -> Mat3 {
            Mat3 {
                m: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            }
        }
    }

    impl Mat2 {
        pub const fn zero() -> Mat2 {
            Mat2 { m: [[0.0; 2]; 2] }
        }
        pub const fn identity() -> Mat2 {
            Mat2 {
                m: [[1.0, 0.0], [0.0, 1.0]],
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        matrix::helpers::Mat4,
        tuples::{helpers::tuple, FEquals},
    };

    use super::helpers::{Mat2, Mat3};

    #[test]
    fn a_4x4_matrix() {
        let m: Mat4 = (
            (1, 2, 3, 4),
            (5.5, 6.5, 7.5, 8.5),
            (9, 10, 11, 12),
            (13.5, 14.5, 15.5, 16.5),
        )
            .into();

        assert!(m[(0, 0)].eps_eq(1.0));
        assert!(m[(0, 3)].eps_eq(4.0));
        assert!(m[(1, 0)].eps_eq(5.5));
        assert!(m[(1, 2)].eps_eq(7.5));
        assert!(m[(2, 2)].eps_eq(11.0));
        assert!(m[(3, 0)].eps_eq(13.5));
        assert!(m[(3, 2)].eps_eq(15.5));
    }

    #[test]
    fn a_2x2_matrix() {
        let m: Mat2 = ((-3, 5), (1, -2)).into();

        assert!(m[(0, 0)].eps_eq(-3.0));
        assert!(m[(0, 1)].eps_eq(5.0));
        assert!(m[(1, 0)].eps_eq(1.0));
        assert!(m[(1, 1)].eps_eq(-2.0));
    }

    #[test]
    fn a_3x3_matrix() {
        let m: Mat3 = ((-3, 5, 0), (1, -2, -7), (0, 1, 1)).into();

        assert!(m[(0, 0)].eps_eq(-3.0));
        assert!(m[(1, 1)].eps_eq(-2.0));
        assert!(m[(2, 2)].eps_eq(1.0));
    }

    #[test]
    fn matrix_equality_identical() {
        let a: Mat4 = ((1, 2, 3, 4), (5, 6, 7, 8), (9, 8, 7, 6), (5, 4, 3, 2)).into();
        let b: Mat4 = ((1, 2, 3, 4), (5, 6, 7, 8), (9, 8, 7, 6), (5, 4, 3, 2)).into();

        assert_eq!(a, b);
    }

    #[test]
    fn matrix_equality_different() {
        let a: Mat4 = ((1, 2, 3, 4), (5, 6, 7, 8), (9, 8, 7, 6), (5, 4, 3, 2)).into();
        let b: Mat4 = ((2, 3, 4, 5), (6, 7, 8, 9), (8, 7, 6, 5), (4, 3, 2, 1)).into();

        assert_ne!(a, b);
    }

    #[test]
    fn mul_matrices() {
        let a: Mat4 = ((1, 2, 3, 4), (5, 6, 7, 8), (9, 8, 7, 6), (5, 4, 3, 2)).into();
        let b: Mat4 = ((-2, 1, 2, 3), (3, 2, 1, -1), (4, 3, 6, 5), (1, 2, 7, 8)).into();

        assert_eq!(
            a * b,
            (
                (20, 22, 50, 48),
                (44, 54, 114, 108),
                (40, 58, 110, 102),
                (16, 26, 46, 42)
            )
                .into()
        );
    }

    #[test]
    fn mul_matrix_tuple() {
        let a: Mat4 = ((1, 2, 3, 4), (2, 4, 4, 2), (8, 6, 4, 1), (0, 0, 0, 1)).into();
        let b = tuple(1, 2, 3, 1);

        assert_eq!(a * b, tuple(18, 24, 33, 1));
    }

    #[test]
    fn identity_matrix() {
        let a: Mat4 = ((0, 1, 2, 4), (1, 2, 4, 8), (2, 4, 8, 16), (4, 8, 16, 32)).into();

        assert_eq!(a.clone() * Mat4::identity(), a);
    }

    #[test]
    fn identity_matrix_tuple() {
        let a = tuple(1, 2, 3, 4);

        assert_eq!(Mat4::identity() * a, a);
    }

    #[test]
    fn transposing_matrices() {
        let a: Mat4 = ((0, 9, 3, 0), (9, 8, 0, 8), (1, 8, 5, 3), (0, 0, 5, 8)).into();
        assert_eq!(
            a.transpose(),
            ((0, 9, 1, 0), (9, 8, 8, 0), (3, 0, 5, 5), (0, 8, 3, 8)).into()
        )
    }

    #[test]
    fn transpose_identity() {
        let a = Mat4::identity();

        assert_eq!(a, Mat4::identity());
    }

    #[test]
    fn determant_2x2() {
        let a: Mat2 = ((1, 5), (-3, 2)).into();
        assert!(a.determinant().eps_eq(17.0));
    }

    #[test]
    fn submatrix_3x3() {
        let a: Mat3 = ((1, 5, 0), (-3, 2, 7), (0, 6, -3)).into();

        assert_eq!(a.submatrix(0, 2), ((-3, 2), (0, 6)).into());
    }

    #[test]
    fn submatrix_4x4() {
        let a: Mat4 = ((-6, 1, 1, 6), (-8, 5, 8, 6), (-1, 0, 8, 2), (-7, 1, -1, 1)).into();

        assert_eq!(
            a.submatrix(2, 1),
            ((-6, 1, 6), (-8, 8, 6), (-7, -1, 1)).into()
        );
    }

    #[test]
    fn minor_3x3() {
        let a: Mat3 = ((3, 5, 0), (2, -1, -7), (6, -1, 5)).into();
        let b = a.submatrix(1, 0);
        assert!(b.determinant().eps_eq(25.0));
        assert!(a.minor(1, 0).eps_eq(25.0));
    }

    #[test]
    fn cofactor_3x3() {
        let a: Mat3 = ((3, 5, 0), (2, -1, -7), (6, -1, 5)).into();
        assert!(a.minor(0, 0).eps_eq(-12.0));
        assert!(a.cofactor(0, 0).eps_eq(-12.0));
        assert!(a.minor(1, 0).eps_eq(25.0));
        assert!(a.cofactor(1, 0).eps_eq(-25.0));
    }

    #[test]
    fn determinant_3x3() {
        let a: Mat3 = ((1, 2, 6), (-5, 8, -4), (2, 6, 4)).into();

        assert!(a.cofactor(0, 0).eps_eq(56.0));
        assert!(a.cofactor(0, 1).eps_eq(12.0));
        assert!(a.cofactor(0, 2).eps_eq(-46.0));

        assert!(a.determinant().eps_eq(-196.0));
    }

    #[test]
    fn determinant_4x4() {
        let a: Mat4 = ((-2, -8, 3, 5), (-3, 1, 7, 3), (1, 2, -9, 6), (-6, 7, 7, -9)).into();
        assert!(a.cofactor(0, 0).eps_eq(690.0));
        assert!(a.cofactor(0, 1).eps_eq(447.0));
        assert!(a.cofactor(0, 2).eps_eq(210.0));
        assert!(a.cofactor(0, 3).eps_eq(51.0));
        assert!(a.determinant().eps_eq(-4071.0));
    }

    #[test]
    fn testing_for_invertibility() {
        {
            let a: Mat4 = ((6, 4, 4, 4), (5, 5, 7, 6), (4, -9, 3, -7), (9, 1, 7, -6)).into();
            assert!(a.determinant().eps_eq(-2120.0));
            assert!(a.invertible());
        }

        {
            let a: Mat4 = ((-4, 2, -2, -3), (9, 6, 2, 6), (0, -5, 1, -5), (0, 0, 0, 0)).into();
            assert!(a.determinant().eps_eq(0.0));
            assert!(!a.invertible());
        }
    }

    #[test]
    fn inverse_4x4() {
        let a: Mat4 = ((-5, 2, 6, -8), (1, -5, 1, 8), (7, 7, -6, -7), (1, -3, 7, 4)).into();
        let b = a.inverse();
        assert!(a.determinant().eps_eq(532.0));
        assert!(a.cofactor(2, 3).eps_eq(-160.0));
        assert!(b[(3, 2)].eps_eq(-160.0 / 532.0));
        assert!(a.cofactor(3, 2).eps_eq(105.0));
        assert!(b[(2, 3)].eps_eq(105.0 / 532.0));
        assert_eq!(
            b,
            (
                (0.21805, 0.45113, 0.24060, -0.04511),
                (-0.80827, -1.45677, -0.44361, 0.52068),
                (-0.07895, -0.22368, -0.05263, 0.19737),
                (-0.52256, -0.81391, -0.30075, 0.30639)
            )
                .into()
        )
    }

    #[test]
    fn more_inverse() {
        {
            let a: Mat4 = ((8, -5, 9, 2), (7, 5, 6, 1), (-6, 0, 9, 6), (-3, 0, -9, -4)).into();

            assert_eq!(
                a.inverse(),
                (
                    (-0.15385, -0.15385, -0.28205, -0.53846),
                    (-0.07692, 0.12308, 0.02564, 0.03077),
                    (0.35897, 0.35897, 0.43590, 0.92308),
                    (-0.69231, -0.69231, -0.76923, -1.92308)
                )
                    .into()
            );
        }

        {
            let a: Mat4 = ((9, 3, 0, 9), (-5, -2, -6, -3), (-4, 9, 6, 4), (-7, 6, 6, 2)).into();

            assert_eq!(
                a.inverse(),
                (
                    (-0.04074, -0.07778, 0.14444, -0.22222),
                    (-0.07778, 0.03333, 0.36667, -0.33333),
                    (-0.02901, -0.14630, -0.10926, 0.12963),
                    (0.17778, 0.06667, -0.26667, 0.33333)
                )
                    .into()
            )
        }
    }

    #[test]
    fn mul_inverse() {
        let a: Mat4 = ((3, -9, 7, 2), (3, -8, 2, -9), (-4, 4, 4, 1), (-6, 5, -1, 1)).into();
        let b: Mat4 = ((8, 2, 2, 2), (3, -1, 7, 0), (7, 0, 5, 4), (6, -2, 0, 5)).into();

        let c = a.clone() * b.clone();

        assert_eq!(c * b.inverse(), a);
    }

    #[test]
    fn printing_matrices() {
        let a: Mat4 = ((3, -9, 7, 2), (3, -8, 2, -9), (-4, 4, 4, 1), (-6, 5, -1, 1)).into();
        println!("{a:?}");
    }
}
