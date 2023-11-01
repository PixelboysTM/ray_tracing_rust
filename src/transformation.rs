use crate::matrix::helpers::Mat4;

pub fn translation(x: f64, y: f64, z: f64) -> Mat4 {
    Mat4::new(
        1.0, 0.0, 0.0, x, 0.0, 1.0, 0.0, y, 0.0, 0.0, 1.0, z, 0.0, 0.0, 0.0, 1.0,
    )
}
pub fn scaling(x: f64, y: f64, z: f64) -> Mat4 {
    Mat4::new(
        x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0, 0.0, 1.0,
    )
}

pub fn rotation_x(rad: f64) -> Mat4 {
    Mat4::new(
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        rad.cos(),
        -(rad.sin()),
        0.0,
        0.0,
        rad.sin(),
        rad.cos(),
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    )
}

pub fn rotation_y(rad: f64) -> Mat4 {
    Mat4::new(
        rad.cos(),
        0.0,
        rad.sin(),
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        -rad.sin(),
        0.0,
        rad.cos(),
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    )
}

pub fn rotation_z(rad: f64) -> Mat4 {
    Mat4::new(
        rad.cos(),
        -rad.sin(),
        0.0,
        0.0,
        rad.sin(),
        rad.cos(),
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    )
}

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Mat4 {
    Mat4::new(
        1.0, xy, xz, 0.0, yx, 1.0, yz, 0.0, zx, zy, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    )
}

pub const PI: f64 = std::f64::consts::PI;

pub mod helper {
    use crate::matrix::helpers::Mat4;

    use super::{rotation_x, rotation_y, rotation_z, scaling, shearing, translation};

    pub struct TransformationBuilder {
        transform: Mat4,
    }

    impl TransformationBuilder {
        pub fn create() -> Self {
            Self {
                transform: Mat4::identity(),
            }
        }
        pub fn translation(mut self, x: f64, y: f64, z: f64) -> Self {
            self.transform = translation(x, y, z) * self.transform;
            self
        }
        pub fn scaling(mut self, x: f64, y: f64, z: f64) -> Self {
            self.transform = scaling(x, y, z) * self.transform;
            self
        }

        pub fn rotation_x(mut self, rad: f64) -> Self {
            self.transform = rotation_x(rad) * self.transform;
            self
        }

        pub fn rotation_y(mut self, rad: f64) -> Self {
            self.transform = rotation_y(rad) * self.transform;
            self
        }

        pub fn rotation_z(mut self, rad: f64) -> Self {
            self.transform = rotation_z(rad) * self.transform;
            self
        }

        pub fn shearing(mut self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
            self.transform = shearing(xy, xz, yx, yz, zx, zy) * self.transform;
            self
        }

        pub fn build(self) -> Mat4 {
            self.transform
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        transformation::{rotation_y, rotation_z, shearing},
        tuples::helpers::{point, vector},
    };

    use super::{helper::TransformationBuilder, rotation_x, scaling, translation, PI};

    #[test]
    fn mul_translation() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = point(-3, 4, 5);

        assert_eq!(transform * p, point(2, 1, 7));
    }

    #[test]
    fn mul_translation_inv() {
        let transform = translation(5.0, -3.0, 2.0);
        let inv = transform.inverse();
        let p = point(-3, 4, 5);

        assert_eq!(inv * p, point(-8, 7, 3));
    }

    #[test]
    fn translation_vector() {
        let transform = translation(5.0, -3.0, 2.0);

        let v = vector(-3, 4, 5);
        assert_eq!(transform * v, v);
    }

    #[test]
    fn scaling_point() {
        let transform = scaling(2.0, 3.0, 4.0);

        let p = point(-4, 6, 8);

        assert_eq!(transform * p, point(-8, 18, 32));
    }

    #[test]
    fn scaling_vector() {
        let transform = scaling(2.0, 3.0, 4.0);

        let v = vector(-4, 6, 8);

        assert_eq!(transform * v, vector(-8, 18, 32));
    }

    #[test]
    fn scaling_inv() {
        let transform = scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse();

        let v = vector(-4, 6, 8);

        assert_eq!(inv * v, vector(-2, 2, 2));
    }

    #[test]
    fn reflecting() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let p = point(2, 3, 4);

        assert_eq!(transform * p, point(-2, 3, 4));
    }

    #[test]
    fn rotate_x() {
        let p = point(0, 1, 0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);

        assert_eq!(
            half_quarter * p,
            point(0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        );

        assert_eq!(full_quarter * p, point(0, 0, 1));
    }

    #[test]
    fn rotate_inv_x() {
        let p = point(0, 1, 0);
        let half_quarter = rotation_x(PI / 4.0);
        let inv = half_quarter.inverse();

        assert_eq!(
            inv * p,
            point(0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn rotate_y() {
        let p = point(0, 0, 1);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);

        assert_eq!(
            half_quarter * p,
            point(2.0_f64.sqrt() / 2.0, 0, 2.0_f64.sqrt() / 2.0)
        );

        assert_eq!(full_quarter * p, point(1, 0, 0));
    }

    #[test]
    fn rotate_z() {
        let p = point(0, 1, 0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);

        assert_eq!(
            half_quarter * p,
            point(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0)
        );

        assert_eq!(full_quarter * p, point(-1, 0, 0));
    }

    #[test]
    fn sheraing_x_in_y() {
        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2, 3, 4);
        assert_eq!(transform * p, point(5, 3, 4));
    }

    #[test]
    fn sheraing_x_in_z() {
        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2, 3, 4);
        assert_eq!(transform * p, point(6, 3, 4));
    }

    #[test]
    fn sheraing_y_in_x() {
        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = point(2, 3, 4);
        assert_eq!(transform * p, point(2, 5, 4));
    }

    #[test]
    fn sheraing_y_in_z() {
        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = point(2, 3, 4);
        assert_eq!(transform * p, point(2, 7, 4));
    }

    #[test]
    fn sheraing_z_in_x() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = point(2, 3, 4);
        assert_eq!(transform * p, point(2, 3, 6));
    }

    #[test]
    fn sheraing_z_in_y() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = point(2, 3, 4);
        assert_eq!(transform * p, point(2, 3, 7));
    }

    #[test]
    fn transform_in_sequence() {
        let p = point(1, 0, 1);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        let p2 = a * p;
        assert_eq!(p2, point(1, -1, 0));

        let p3 = b * p2;
        assert_eq!(p3, point(5, -5, 0));

        let p4 = c * p3;
        assert_eq!(p4, point(15, 0, 7));
    }

    #[test]
    fn chained_in_reverse() {
        let p = point(1, 0, 1);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        let t = c * b * a;
        assert_eq!(t * p, point(15, 0, 7));
    }

    #[test]
    fn chaining_in_builder() {
        let t = TransformationBuilder::create()
            .rotation_x(PI / 2.0)
            .scaling(5.0, 5.0, 5.0)
            .translation(10.0, 5.0, 7.0)
            .build();
        let p = point(1, 0, 1);

        assert_eq!(t * p, point(15, 0, 7));
    }
}
