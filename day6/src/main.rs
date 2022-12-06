use std::{collections::HashMap, fs};

const DELIMITER: usize = 14;
fn main() {
    let signal = parse_input();
    let res = parse_signal(signal);
    println!("{}", res);
}

fn parse_input() -> String {
    return fs::read_to_string("input.txt").expect("Error reading file");
}

fn parse_signal(signal: String) -> usize {
    for i in 0..signal.len() {
        let mut map: HashMap<char, i32> = HashMap::new();
        let part = &signal[i..i + DELIMITER];
        for c in part.chars() {
            map.insert(c, 0);
        }
        if map.len() == DELIMITER {
            return i + DELIMITER;
        }
    }
    return 0;
}
