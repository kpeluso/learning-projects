/// @title Vose Algorithm
/// @author Kenny Peluso

extern crate rand;
extern crate num_traits;

mod vose;

fn main() {
    println!("Running Vose...");
    println!("{}", vose::vose(&vec![0.4,0.5,0.1]));
}
