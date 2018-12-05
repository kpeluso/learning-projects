use std::io;

fn fact(n: u32) -> u32 {
    // recursive
    match n {
        0|1 => 1,
        _ => n*fact(n-1),
    }
}

fn main() {
    println!("Factorial >>> ");
    let mut u = String::new();
    io::stdin()
        .read_line(&mut u)
        .expect("Can't read input!");
    let n = u.trim().parse::<u32>().expect("Not a natural number!");
    println!("\nOutput:\n{}", fact(n));
}
