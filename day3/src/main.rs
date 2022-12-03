use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut total_sum = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(rucksack) = line {
                // Define an hashmap to store all items, initialize it with the first compartment
                let mut all_items = HashMap::new();
                let size = rucksack.len();
                let first_compartment = &rucksack[..(size/2)];
                for item in first_compartment.chars() {
                    all_items.insert(item, 0);
                }
                // Iterate over the second compartment and check if the item is already in the hashset
                let second_compartment = &rucksack[(size/2)..];
                for item in second_compartment.chars() {
                    // If the item is already in the hashset, we add the priority of the item to the total sum and break the loop
                    if all_items.contains_key(&item) {
                        println!("{} - {}", item, get_item_priority(item));
                        total_sum += get_item_priority(item);
                        break;
                    }
                }
            }
        }
    }
    println!("Total sum: {}", total_sum);
}

fn get_item_priority(item: char) -> u32 {
    let ascii_code = item as u32;
    if ascii_code > 96 {
        return ascii_code - 96;
    } else {
        return ascii_code - 38;
    }
}

// Read input file, taken from rust-by-example
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
