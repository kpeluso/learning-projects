use std::io;
mod calc;

fn read(s: &str) -> f64 {
    println!("{}", s);
    let mut u = String::new();
    io::stdin()
        .read_line(&mut u)
        .expect("Can't read input!");
    let inp = u.trim().parse::<f64>().unwrap() ;
    return inp;
}

fn main() {
    let n: f64 = read("Enter first number >>>");
    let m: f64 = read("Enter second number >>>");
    match read("Enter a calculation (1-add, 2-sub, 3-mul, 4-div) >>> ") as u8 {
        1 => println!("{}", calc::add::<f64>(n,m)),
        2 => println!("{}", calc::sub::<f64>(n,m)),
        3 => println!("{}", calc::mul::<f64>(n,m)),
        4 => println!("{}", calc::div::<f64>(n,m)),
        _ => panic!("{}", "Must enter 1, 2, 3, or 4 - nothing else..."),
    };
}
