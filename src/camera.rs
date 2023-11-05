use std::arch::x86_64;

use crate::{
    canvas::Canvas, matrix::helpers::Mat4, ray::Ray, tuples::helpers::point, world::World,
};

pub struct Camera {
    hsize: usize,
    vsize: usize,
    fov: f64,
    transform: Mat4,
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, fov: f64) -> Camera {
        let half_view = (fov / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;

        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        Camera {
            hsize,
            vsize,
            fov,
            transform: Mat4::identity(),
            half_height,
            half_width,
            pixel_size: (half_width * 2.0) / hsize as f64,
        }
    }
    pub fn new_transformed(hsize: usize, vsize: usize, fov: f64, transform: Mat4) -> Camera {
        let mut c = Camera::new(hsize, vsize, fov);
        c.transform = transform;
        c
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let inv = self.transform().inverse();

        let pixel = inv.clone() * point(world_x, world_y, -1);
        let origin = inv * point(0, 0, 0);
        let direction = (pixel - origin).normalized();

        Ray::new(origin, direction)
    }
    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray);
                image[(x, y)] = color;
            }
        }

        image
    }

    pub fn hsize(&self) -> usize {
        self.hsize
    }
    pub fn vsize(&self) -> usize {
        self.vsize
    }
    pub fn fov(&self) -> f64 {
        self.fov
    }
    pub fn pixel_size(&self) -> f64 {
        self.pixel_size
    }
    pub fn transform(&self) -> &Mat4 {
        &self.transform
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        canvas::Canvas,
        matrix::helpers::Mat4,
        ray::Ray,
        transformation::{rotation_y, translation, view_transform, PI},
        tuples::{
            helpers::{color, point, vector},
            FEquals,
        },
        world::World,
    };

    use super::Camera;

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let fov = PI / 2.0;

        let c = Camera::new(hsize, vsize, fov);

        assert_eq!(c.hsize(), 160);
        assert_eq!(c.vsize(), 120);
        assert!(c.fov().eps_eq(PI / 2.0));
        assert_eq!(c.transform(), &Mat4::identity());
    }

    #[test]
    fn pixel_size_h() {
        let c = Camera::new(200, 125, PI / 2.0);
        assert!(c.pixel_size().eps_eq(0.01));
    }

    #[test]
    fn pixel_size_v() {
        let c = Camera::new(125, 200, PI / 2.0);
        assert!(c.pixel_size().eps_eq(0.01));
    }

    #[test]
    fn ray_center() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r: Ray = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin(), point(0, 0, 0));
        assert_eq!(r.direction(), vector(0, 0, -1));
    }

    #[test]
    fn ray_corner() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r: Ray = c.ray_for_pixel(0, 0);
        assert_eq!(r.origin(), point(0, 0, 0));
        assert_eq!(r.direction(), vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn ray_transformed() {
        let c = Camera::new_transformed(
            201,
            101,
            PI / 2.0,
            rotation_y(PI / 4.0) * translation(0.0, -2.0, 5.0),
        );
        let r: Ray = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin(), point(0, 2, -5));
        assert_eq!(
            r.direction(),
            vector(2.0_f64.sqrt() / 2.0, 0.0, -2.0_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn rendering_world() {
        let w = World::default();

        let c = Camera::new_transformed(
            11,
            11,
            PI / 2.0,
            view_transform(point(0, 0, -5), point(0, 0, 0), vector(0, 1, 0)),
        );

        let image: Canvas = c.render(&w);

        assert_eq!(image[(5, 5)], color(0.38066, 0.47583, 0.2855));
    }
}
