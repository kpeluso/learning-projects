use std::ops::{Add, Sub, Mul, Div};

// Source: https://stackoverflow.com/questions/27932655/how-can-i-write-a-generic-function-that-discriminates-between-signed-and-unsigne
pub fn add<T: Add<Output = T>>(n:T, m:T) -> T {
    println!("addition");
    return n + m;
}
pub fn sub<T: Sub<Output = T>>(n:T, m:T) -> T {
    println!("subtraction");
    // if m > n && !int::is_signed::<T> { panic!("AAAaaaaa!!!!"); }
    return n - m;
}

pub fn mul<T: Mul<Output = T>>(n:T, m:T) -> T {
    println!("multiplication");
    return n * m;
}

pub fn div<T: Div<Output = T>>(n:T, m:T) -> T {
    println!("division");
    return n/m;
}
