use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;

fn normalize_dict() {
    println!("normalize dict");
}

fn concat_ws(s1: String, s2: String) -> String {
    // Source: https://stackoverflow.com/questions/30154541/how-do-i-concatenate-strings
    let owned_string: String = s1.to_owned();
    let borrowed_string: &str = &s2;
    let together = owned_string.clone() + " " +  borrowed_string;
    return together
}

fn format(w: &str) -> String {
    return w.to_string()
            .trim()
            .to_lowercase()
            .trim_matches(|c| c == ',' || c == '?' || c == '\'' || c == '.')
            .to_string();
}

// Help: https://stackoverflow.com/questions/34969902/how-to-write-a-rust-function-that-takes-an-iterator
fn iter_assign<'a, I>(mut iter: I) -> String
where
    I: Iterator<Item = &'a str>,
{
    match iter.next() {
        Some(w) => return format(w),
        None => panic!("Ended early! File too short!"),
    }
}

fn ma(s: String, d: std::path::Display) -> HashMap<String, HashMap<String, u32>> {
    println!("\nMarkov Analyzing: {}, which contains:\n\n{}", d, s);
    let mut table: HashMap<String, HashMap<String, u32>> = HashMap::new();
    let iter = s.split_whitespace();
    let mut prefix1: String = iter_assign(iter.clone());
    let mut prefix2: String = iter_assign(iter.clone());
    let mut key: String;
    let mut ns: String;
    for new_suffix in iter {
        key = concat_ws(prefix1, prefix2.clone());
        ns = format(new_suffix);
        // update a key, guarding against the key possibly not being set
        let a_row = table.entry(key).or_insert(HashMap::new());
        let stat = a_row.entry(ns.clone()).or_insert(0);
        *stat += 1;
        prefix1 = prefix2;
        prefix2 = ns.clone();
    }
    return table;
}

fn main() {
    // Create a path to the desired file
    let path = Path::new("p17/src/text.txt");
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
        // Ok(_) => println!("{} contains:\n{}", display, s),
        Ok(_) => ma(s, display),
    };

    // `file` goes out of scope, and the "hello.txt" file gets closed

    //
    // √ (done) MAKE KEYS AND CONTENTS LOWERCASE AND REMOVE PUNCTUATION
    //

    match m.get("half the") {
        Some(row) => {
            match row.get("bee") {
                Some(val) => println!("NUMBER: half the bee: {}", val),
                None => println!("NOPE! #2"),
            }
        },
        None => println!("NOPE! #1"),
    }
}
