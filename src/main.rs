mod canvas;
mod tuples;

macro_rules! time {
    ($b:block) => {{
        let xx_timer_bb_dnsajkfdasb = std::time::Instant::now();
        $b;
        xx_timer_bb_dnsajkfdasb.elapsed()
    }};
}

fn main() {
    println!("Hello World");
    pit_01::main();

    let time = time!({
        pit_02::main();
    });
    println!("{time:#?}");
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
