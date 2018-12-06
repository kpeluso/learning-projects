use std::io;
fn main() {
    println!("Enter a natrual number >>> ");
    let mut u = String::new();
    io::stdin()
        .read_line(&mut u)
        .expect("Can't read input!");
    let mut inp = u.trim().parse::<u32>().expect("Not an integer!");
    let mut c: u32;
    while inp > 0 {
        c = 0;
        while c < inp {
            print!("*");
            c += 1;
        }
        print!("\n");
        inp -= 1;
    }
}
