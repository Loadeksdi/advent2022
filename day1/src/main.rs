use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut total_calories: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;
    // Iterate on each line of the file
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(calories) = line {
                // If the line is empty, we have reached the end of an elf calorie list,
                // so we add the sum to the total calories vector and reset the sum to 0
                if calories == "" {
                    total_calories.push(sum);
                    sum = 0;
                }
                // Otherwise, we add the calories to the sum
                else {
                    sum += calories.parse::<i32>().unwrap();
                }
            }
        }
    }
    let max_val: &i32 = total_calories.iter().max().unwrap();
    println!("Max calories: {}", max_val);
}

// Read input file, taken from rust-by-example
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
