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
    let mut top_three_calories: i32 = 0;
    let mut i = 0;
    while i < 3{
        // Find the index of the maximum value in the total calories vector
        let index_of_max: Option<usize> = total_calories
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index);
        // Remove the maximum value from the total calories vector and add it to the top three calories
        let val:i32 = total_calories.swap_remove(index_of_max.unwrap());
        top_three_calories += val;
        i += 1;
    }
    println!("The top three calories are: {}", top_three_calories);
}

// Read input file, taken from rust-by-example
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
