extern crate rand;
extern crate num_traits;

use num_traits::cast::ToPrimitive;

use std::convert::From;
use rand::Rng;

/// @notice pure
/// @return scalar multiplication of real `a` and vector `v`
fn sm<T, U>(a: T, v: &Vec<U>) -> Vec<T>
    where U: ToPrimitive, T: ToPrimitive + From<f64> {
    let mut out: Vec<T> = Vec::new();
    for i in v {
        out.push(T::from(ToPrimitive::to_f64(i).unwrap() * ToPrimitive::to_f64(&a).unwrap()))
    }
    return out
}

/// @return alias, prob construction via Vose Algorithm
/// @dev v1 - just rebuild alias/prob every call
fn init(v: &Vec<f64>) -> (Vec<usize>, Vec<f64>) {
    let n = (*v).len();
    let na = n as f64;
    let mut j: usize;
    let mut k: usize;
    let mut vv: Vec<f64> = vec![0.0; n];
    vv.clone_from_slice(&v);
    let mut alias: Vec<usize> = vec![0; n];
    let mut probs: Vec<f64> = vec![0.0; n];
    let mut small: Vec<usize> = vec![0; n];
    let mut large: Vec<usize> = vec![0; n];
    
    let mut l = 0; let mut s = 0;
    for j in 0..n {
        if vv[j] > 1.0/na {
            large[l] = j; l += 1;
        } else {
            small[s] = j; s += 1;
        }
    }
    while s > 0 && l > 0 {
        s -= 1; j = small[s];
        l -= 1; k = large[l];
        probs[j] = na * vv[j];
        alias[j] = k;
        vv[k] = (vv[k] + vv[j]) - 1.0/na;
        if vv[k] > 1.0/na {
            large[l] = k; l += 1;
        } else {
            small[s] = k; s += 1;
        }
    }
    while s > 0 {
        s -= 1; probs[small[s]] = 1.0
    }
    while l > 0 {
        l -= 1; probs[large[l]] = 1.0
    }
    return (alias, probs)
}

/// @return Sample from categorical distribution `a`
/// @dev v1 - just rebuilds alias/prob every call
fn rand(a: &Vec<usize>, p: &Vec<f64>) -> usize {
    let mut rng = rand::thread_rng();
    let mut r: f64 = rng.gen();
    r *= a.len() as f64;
    let j: usize = r as usize;
    if r - (j as f64) <= p[j] {
        return j
    }
    return a[j]
}

/// @return Sample once from categorical distribution `v`
pub fn vose(v: &Vec<f64>) -> usize {
    let (a, p) = init(v);
    return rand(&a, &p)
}

/// @return Sample `n` times from categorical distribution `v`
pub fn vose_n(v: &Vec<f64>, n: usize) -> Vec<usize> {
    let mut out: Vec<usize> = Vec::new();
    let (a, p) = init(v);
    for _ in 0..n {
        out.push(rand(&a, &p))
    }
    return out;
}

/// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_setup() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn monte_carlo() {
        let TOL: f64 = 0.01;
        let ITE: usize = 10000;
        let v = vec![0.15, 0.45, 0.4]; // "random" @dev generate with oracle `m`-times over
        let LEN: usize = v.len();
        let ls: Vec<usize> = vose_n(&v, ITE);
        let mut amt: Vec<usize> = vec![0; LEN];
        for i in ls {
            amt[i] += 1
        }
        let s: usize = amt.iter().sum();
        for i in 0..LEN {
            assert!(((amt[i] as f64)/(s as f64) - v[i]).abs() < TOL);
        }
    }
}
