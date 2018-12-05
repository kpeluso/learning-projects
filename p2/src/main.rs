use std::io;

const X_TO_Y: f64 = 0.8;
const Y_TO_X: f64 = 1.0/0.8;

fn convert(amt: f64, start: u8) -> f64 {
    if start == 1 {
        return X_TO_Y*amt;
    }
    return Y_TO_X*amt;
}

fn main() {
    println!("Start currency (1 for X, 2 for Y) >>> ");
    let mut u = String::new();
    io::stdin()
        .read_line(&mut u)
        .expect("Can't read input!");
    let sc = u.trim().parse::<u8>().expect("Not an integer!");
    match sc {
        1|2 => {
            println!("How much would you like to convert? (enter float amount)");
            let mut u2 = String::new();
            io::stdin()
                .read_line(&mut u2)
                .expect("Can't read input!");
            let amt: f64 = u2.trim().parse::<f64>().expect("Not a float!");
            println!("\nOutput:\n{:.2}", convert(amt, sc));
        },
        _ => println!("You can only enter x or y")
    }
}
