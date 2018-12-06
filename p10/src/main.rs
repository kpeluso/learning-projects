fn main() {
    let mut t: u32 = 0;
    for i in 1..100 {
        if i%2 == 1 {
            t += i*i
        }
    }
    println!("{}", (t as f64).powf(1f64/5f64));
}
