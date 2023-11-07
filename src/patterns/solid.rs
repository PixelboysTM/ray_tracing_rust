use crate::{matrix::helpers::Mat4, tuples::Tuple};

use super::Pattern;

#[derive(Debug)]
pub struct Solid {
    m: Mat4,
    color: Tuple,
}

impl Solid {
    pub const fn new(color: Tuple) -> Solid {
        Self {
            m: Mat4::identity(),
            color,
        }
    }
}

impl Pattern for Solid {
    fn at(&self, _: &Tuple) -> Tuple {
        self.color
    }

    fn transform(&self) -> &Mat4 {
        &self.m
    }

    fn set_transform(&mut self, _: Mat4) {}
}
