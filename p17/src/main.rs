use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use std::option::Option;

mod samp;

fn normalize_dict(dref: &HashMap<String, HashMap<String, f64>>) -> HashMap<String, HashMap<String, f64>> {
    let mut table: HashMap<String, HashMap<String, f64>> = HashMap::new();
    let mut row_total: f64;
    let mut new_row: HashMap<String, f64>;
    for (prefix, a_row) in dref {
        row_total = 0.0;
        for (_, freq) in a_row {
            row_total += freq;
        }
        new_row = HashMap::new();
        for (suffix, freq) in a_row {
            new_row.insert(suffix.to_string(), freq/row_total);
        }
        table.insert(prefix.to_string(), new_row);
    }
    return table;
}

fn concat_ws(s1: String, s2: String) -> String {
    // Source: https://stackoverflow.com/questions/30154541/how-do-i-concatenate-strings
    let owned_string: String = s1.to_owned();
    let borrowed_string: &str = &s2;
    let together = owned_string.clone() + " " +  borrowed_string;
    return together
}

fn concat_ws_vec(v: Vec<String>) -> String {
    let mut output: String = v[0].clone();
    for el in v {
        if el == output { continue; };
        output = concat_ws(output, el);
    }
    return output;
}

fn format(w: &str) -> String {
    return w.to_string()
            .trim()
            .to_lowercase()
            .trim_matches(|c| c == ',' || c == '?' || c == '\'' || c == '.')
            .to_string();
}

// pl == any u32
fn ma_var(s: String, d: std::path::Display, pl: u32) -> HashMap<String, HashMap<String, f64>> {
    println!("\nMarkov Analyzing: {}, which contains:\n\n{}", d, s);
    if (s.len() as u32) < pl+1 { panic!("File text is not long enough (or prefix length is too long)!"); }
    let mut table: HashMap<String, HashMap<String, f64>> = HashMap::new();
    let mut iter = s.split_whitespace();
    let mut prefixes: Vec<String> = Vec::new();
    let mut new_prefixes: Vec<String> = Vec::new();
    let mut content: String;
    for _ in 0..pl { prefixes.push(Option::unwrap(iter.next()).to_string()); }
    let mut key: String;
    let mut ns: String;
    for new_suffix in iter {
        key = concat_ws_vec(prefixes.clone());
        ns = format(new_suffix);
        // update a key, guarding against the key possibly not being set
        let a_row = table.entry(key).or_insert(HashMap::new());
        let stat = a_row.entry(ns.clone()).or_insert(0.0);
        *stat += 1.0;
        for i in 1..pl {
            content = prefixes[i as usize].clone();
            new_prefixes.push(content);
        }
        new_prefixes.push(ns.clone());
        prefixes = new_prefixes;
        new_prefixes = Vec::new();
    }
    return table;
}

fn write_once(sref: &String, dref: &HashMap<String, HashMap<String, f64>>) -> String {
    match dref.get(sref) {
        Some(row) => return samp::categorical(&row),
        None => panic!("{} is not in the Markov dictionary!", sref.to_string()),
    }
}

fn vslice_2_prefix(v: Vec<String>, c: u32, pl: u32) -> String {
    if v.len() == 0_usize || pl == 0_u32 { return "".to_string(); }
    let mut output: String = (&v[c as usize]).to_string();
    for i in (1+c)..(pl+c) {
        output = concat_ws(output, (&v[i as usize]).to_string());
    }
    return output
}

fn write(n: u32, seed: String, dref: &HashMap<String, HashMap<String, f64>>, pl: u32) {
    let mut c: u32 = 0;
    let mut iter = seed.split_whitespace();
    let mut words: Vec<String> = Vec::new();
    for _ in 0..pl { words.push(Option::unwrap(iter.next()).to_string()); }
    let mut idk: String;
    while c < (n-pl) {
        idk = vslice_2_prefix(words.clone(), c, pl);
        words.push(write_once(&idk, dref));
        c += 1;
    }
    print!("\nGenerated {}-word text:\n{}\n", n, concat_ws_vec(words));
}

fn main() {
    // Parameters
    let steps: u32 = 10; // number of Markov steps of narrative generation to run
    let pl: u32 = 3; // prefix length

    // Create a path to the desired file
    /* let path = Path::new("p17/src/text.txt"); */ // without compilation with external crate
    let path = Path::new("text.txt"); // with compilation with external crate
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string with error
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    let m = match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => ma_var(s, display, pl),
    };

    match m.get("the bee") {
        Some(row) => {
            match row.get("has") {
                Some(val) => println!("NUMBER: half the bee: {}", val),
                None => println!("NOPE! #2a"),
            }
        },
        None => println!("NOPE! #1a"),
    }

    println!("'the bee' support:");
    match m.get("the bee") {
        Some(support) => {
            for (pre, suf) in support {
                println!("{} :: {}", pre, suf);
            }
        },
        None => println!("NOPE! #1c"),
    }

    let q = normalize_dict(&m);

    match q.get("the bee") {
        Some(row) => {
            match row.get("has") {
                Some(val) => println!("NUMBER: half the bee: {}", val),
                None => println!("NOPE! #2b"),
            }
        },
        None => println!("NOPE! #1b"),
    }

    // generate a narrative with given data
    let seed: String;
    for (prefix, _) in &q {
        seed = prefix.to_string();
        println!("The seed is: '{}'", prefix);
        write(steps, seed, &q, pl);
        break;
    }

    // `file` goes out of scope, and the "text.txt" file gets closed
}
