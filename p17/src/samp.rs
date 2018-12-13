extern crate rand;

use std::collections::HashMap;
use self::rand::Rng;

fn cumulate(vref: &HashMap<String, f64>) -> HashMap<String, f64> {
    let mut output: HashMap<String, f64> = HashMap::new();;
    let mut prev: f64 = 0.0;
    for (key, val) in vref {
        prev += val;
        output.insert(key.to_string(), prev);
    }
    return output;
}

pub fn categorical(v: &HashMap<String, f64>) -> String {
    // given weight vector `v` output index of sample
    let mut rng = rand::thread_rng();
    let r: f64 = rng.gen(); // generates a float between 0 and 1
    let vc: HashMap<String, f64> = cumulate(v);
    let mut ans: String = "".to_string();
    for (key, val) in vc {
        ans = key.to_string();
        if val > r {
            break;
        }
    }
    return ans.to_string();
}
