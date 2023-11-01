mod canvas;
mod matrix;
mod transformation;
mod tuples;

fn main() {
    println!("Hello World");
    pit_01::main();
    pit_02::main();
    pit_03::main();
    pit_04::main();
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
    use crate::matrix::helpers::{Mat2, Mat4};

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
