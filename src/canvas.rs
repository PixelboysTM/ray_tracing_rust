use std::ops::{Index, IndexMut};

use image::{DynamicImage, GenericImage, ImageResult, Rgba};

use crate::tuples::{helpers::colors, Tuple};

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Tuple>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let size = width * height;

        Canvas {
            width,
            height,
            pixels: Vec::from_iter((0..size).map(|_| colors::black())),
        }
    }

    pub fn save(&self, path: &str) -> ImageResult<()> {
        let mut img = DynamicImage::new_rgba8(self.width as u32, self.height as u32);

        let a = 255;

        (0..self.width).into_iter().for_each(|x| {
            (0..self.height).into_iter().for_each(|y| {
                let pixel = &self[(x, y)];

                let r = (pixel.r() * 255.0) as u8;
                let g = (pixel.g() * 255.0) as u8;
                let b = (pixel.b() * 255.0) as u8;

                img.put_pixel(x as u32, y as u32, Rgba([r, g, b, a]));
            })
        });

        img.save(path)
    }
}

impl Index<(usize, usize)> for Canvas {
    type Output = Tuple;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        let index = x * self.height + y;
        &self.pixels[index]
    }
}

impl IndexMut<(usize, usize)> for Canvas {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Tuple {
        let index = x * self.height + y;
        &mut self.pixels[index]
    }
}

#[cfg(test)]
mod tests {

    use crate::tuples::helpers::{color, colors};

    use super::*;

    #[test]
    fn creating_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);

        for x in 0..10 {
            for y in 0..20 {
                assert_eq!(c[(x, y)], colors::black())
            }
        }
    }

    #[test]
    fn writing_to_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = color(1, 0, 0);

        c[(2, 3)] = red;

        assert_eq!(c[(2, 3)], red);
    }

    #[test]
    fn writing_to_disk() {
        let c = Canvas::new(10, 20);

        c.save("./temp/test.png").expect("Saving image failed");
    }
}
