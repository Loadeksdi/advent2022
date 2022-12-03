use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;


fn main() {
    // Insert each possible rock paper scissors score into a hashmap
    let mut scores = HashMap::new();
    scores.insert(String::from("A X"), 4);
    scores.insert(String::from("A Y"), 8);
    scores.insert(String::from("A Z"), 3);
    scores.insert(String::from("B X"), 1);
    scores.insert(String::from("B Y"), 5);
    scores.insert(String::from("B Z"), 9);
    scores.insert(String::from("C X"), 7);
    scores.insert(String::from("C Y"), 2);
    scores.insert(String::from("C Z"), 6);
    let mut total_score: i32 = 0;
    // Iterate on each line of the file
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(game) = line {
               total_score += scores[&game]; 
            }
        }
    }
    println!("Total score: {}", total_score);
}

// Read input file, taken from rust-by-example
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
