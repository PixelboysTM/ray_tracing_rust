mod canvas;
mod light;
mod material;
mod matrix;
mod ray;
mod sphere;
mod transformation;
mod tuples;

fn main() {
    println!("Hello World");
    pit_01::main();
    pit_02::main();
    pit_03::main();
    pit_04::main();
    pit_05::main();
    pit_06::main();
}

mod pit_01 {
    use crate::tuples::{
        helpers::{point, vector},
        Tuple,
    };

    fn tick(env: (Tuple, Tuple), proj: (Tuple, Tuple)) -> (Tuple, Tuple) {
        let position = proj.0 + proj.1;
        let velocity = proj.1 + env.0 + env.1;
        return (position, velocity);
    }

    pub fn main() {
        let mut p = (point(0, 1, 0), vector(1, 1, 0).normalized());
        let e = (vector(0, -0.1, 0), vector(-0.01, 0, 0));

        let mut ticker = 0;
        loop {
            ticker += 1;
            if p.0.y() <= 0.0 {
                break;
            }
            p = tick(e, p);

            println!("Tick {ticker} at {:?}", p.0);
        }

        println!("It took {ticker} Ticks.")
    }
}

mod pit_02 {
    use crate::{
        canvas::Canvas,
        tuples::{
            helpers::{color, point, vector},
            Tuple,
        },
    };

    fn tick(env: (Tuple, Tuple), proj: (Tuple, Tuple)) -> (Tuple, Tuple) {
        let position = proj.0 + proj.1;
        let velocity = proj.1 + env.0 + env.1;
        return (position, velocity);
    }

    pub fn main() {
        let start = point(0, 1, 0);
        let velocity = vector(1, 1.8, 0).normalized() * 11.25;

        let mut p = (start, velocity);

        let gravity = vector(0, -0.1, 0);
        let wind = vector(-0.01, 0, 0);

        let e = (gravity, wind);

        let mut c = Canvas::new(900, 550);

        loop {
            if p.0.y() <= 0.0 {
                break;
            }

            let x = p.0.x().round() as usize;
            let y = 550 - p.0.y().round() as usize;

            c[(x, y)] = color(1, 0, 0);

            p = tick(e, p);
        }

        c.save("./temp/pit_02.png").expect("Unable to save file");
    }
}

mod pit_03 {
    use crate::matrix::helpers::Mat4;

    pub fn main() {
        let a: Mat4 = ((3, -9, 7, 2), (3, -8, 2, -9), (-4, 4, 4, 1), (-6, 5, -1, 1)).into();
        // let a: Mat2 = ((3, -9), (3, -8)).into();
        println!("{a:?}");

        println!("Invert of identity:\n{:?}", Mat4::identity());
        println!("A * inv(A) =\n{:?}", a.clone() * a.inverse());
        println!(
            "{:?}=\n{:?}",
            a.inverse().transpose(),
            a.transpose().inverse()
        );
    }
}

mod pit_04 {
    use crate::{
        canvas::Canvas,
        transformation::rotation_y,
        tuples::helpers::{colors, point},
    };

    pub fn main() {
        let mut c = Canvas::new(100, 100);

        for i in 0..12 {
            let p = point(0, 0, 45);
            let rot = rotation_y((30.0 * i as f64).to_radians());
            let np = rot * p;

            let x = 50.0 + np.x();
            let y = 50.0 + np.z();

            c[(x as usize, y as usize)] = colors::white();
        }

        c.save("./temp/pit_04.png").unwrap();
    }
}

mod pit_05 {

    use crate::{
        canvas::Canvas,
        ray::{Intersections, Ray},
        sphere::Sphere,
        transformation::scaling,
        tuples::helpers::{color, point},
    };

    pub fn main() {
        let ray_origin = point(0, 0, -5);
        let wall_z = 10.0;
        let wall_size = 7.0;

        let canvas_pixel = 800;

        let pixel_size = wall_size / canvas_pixel as f64;

        let half = wall_size / 2.0;

        let mut canvas = Canvas::new(canvas_pixel, canvas_pixel);
        let color = color(1, 0, 0);

        let mut shape = Sphere::new();
        shape.set_transform(scaling(1.0, 0.5, 1.0));

        for y in 0..canvas_pixel - 1 {
            let world_y = half - pixel_size as f64 * y as f64;

            for x in 0..canvas_pixel - 1 {
                let world_x = -half + pixel_size as f64 * x as f64;

                let position = point(world_x, world_y, wall_z);

                let r = Ray::new(ray_origin, (position - ray_origin).normalized());
                let xs = shape.intersect(&r);

                match xs.hit() {
                    Some(_) => canvas[(x, y)] = color,
                    None => {}
                }
            }
        }

        canvas.save("./temp/pit_05.png").unwrap();
    }
}

mod pit_06 {

    use crate::{
        canvas::Canvas,
        light::PointLight,
        material::Material,
        ray::{Intersections, Ray},
        sphere::Sphere,
        transformation::scaling,
        tuples::helpers::{color, point},
    };

    pub fn main() {
        let ray_origin = point(0, 0, -5);
        let wall_z = 10.0;
        let wall_size = 7.0;

        let canvas_pixel = 100;

        let pixel_size = wall_size / canvas_pixel as f64;

        let half = wall_size / 2.0;

        let mut canvas = Canvas::new(canvas_pixel, canvas_pixel);

        let mut shape = Sphere::new();
        // shape.set_transform(scaling(1.0, 0.5, 1.0));
        let mut m = Material::default();
        m.color = color(1, 0.2, 1);
        shape.set_material(m);

        let light = PointLight::new(color(1, 1, 1), point(-10, 10, -10));

        for y in 0..canvas_pixel - 1 {
            let world_y = half - pixel_size as f64 * y as f64;

            for x in 0..canvas_pixel - 1 {
                let world_x = -half + pixel_size as f64 * x as f64;

                let position = point(world_x, world_y, wall_z);

                let r = Ray::new(ray_origin, (position - ray_origin).normalized());
                let xs = shape.intersect(&r);

                match xs.hit() {
                    Some(hit) => {
                        let point = r.at(hit.t());
                        let normal = hit.object().normal_at(point);
                        let eye = -r.direction();

                        let color = hit.object().material().lighting(&light, point, eye, normal);

                        canvas[(x, y)] = color;
                    }
                    None => {}
                }
            }
        }

        canvas.save("./temp/pit_06.png").unwrap();
    }
}
