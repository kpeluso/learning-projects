extern crate rand;

use rand::Rng;

fn x() {
    println!("Hey, it's function X!");
}

fn y() {
    println!("Hey, it's function Y!");
}

fn z() {
    println!("Hey, it's function Z!");
}

fn main() {
    // to run programs with external dependencies: cargo build; cargo run
    let mut rng = rand::thread_rng();
    let r: f64 = rng.gen(); // generates a float between 0 and 1
    if r < 0.333 {
        x();
    } else if 0.333 < r && r < 0.666 {
        y();
    } else {
        z();
    }
}
