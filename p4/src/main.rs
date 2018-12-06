use std::io;
fn main() {
    println!("Enter a natrual number >>> ");
    let mut u = String::new();
    io::stdin()
        .read_line(&mut u)
        .expect("Can't read input!");
    let inp = u.trim().parse::<u32>().expect("Not an integer!");
    let mut c: u32 = 0;
    let mut s: u32;
    while inp > c {
        s = 0;
        while s < c+1 {
            print!("*");
            s += 1;
        }
        print!("\n");
        c = s;
    }
}
