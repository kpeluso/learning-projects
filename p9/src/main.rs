use std::io;
fn main() {
    println!("Enter a string >>> ");
    let mut u = String::new();
    io::stdin()
        .read_line(&mut u)
        .expect("Can't read input!");
    let inp = u.trim();
    if !inp.chars().all(|x| x.is_alphabetic()) {
        println!("Input string must only contain alphabetic characters!");
        return;
    }
    let mut v: u32 = 0;
    for i in inp.chars() {
        match i {
            'a'|'e'|'i'|'o'|'u' => { v += 1; },
            _ => continue,
        }
    }
    println!("{} vowels and {} consonants.", v, inp.len() - (v as usize));
}
