use std::io;
fn main() {
    println!("Enter a natrual number >>> ");
    let mut u = String::new();
    io::stdin()
        .read_line(&mut u)
        .expect("Can't read input!");
    let inp = u.trim()

}
