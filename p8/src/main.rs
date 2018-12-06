use std::io;
fn main() {
    println!("Enter natural number upper bound for Fibonacci sequence >>> ");
    let mut u = String::new();
    io::stdin()
        .read_line(&mut u)
        .expect("Can't read input!");
    let inp = u.trim().parse::<u32>().expect("Not an integer!");
    let mut f: u32 = 1;
    let mut p: u32 = 0;
    let mut oldf: u32;
    while inp > f {
        print!("{} ", f);
        oldf = f;
        f += p;
        p = oldf;
    }
    print!("\n");
}
