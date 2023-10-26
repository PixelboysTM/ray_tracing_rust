mod tuples;

fn main() {
    println!("Hello World");
    pit_01::main();
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
