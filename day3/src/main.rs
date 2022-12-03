use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // part_one();
    part_two();
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

fn part_one() {
    let mut total_sum = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(rucksack) = line {
                // Define an hashmap to store all items, initialize it with the first compartment
                let mut all_items = HashMap::new();
                let size = rucksack.len();
                let first_compartment = &rucksack[..(size / 2)];
                for item in first_compartment.chars() {
                    all_items.insert(item, 0);
                }
                // Iterate over the second compartment and check if the item is already in the hashset
                let second_compartment = &rucksack[(size / 2)..];
                for item in second_compartment.chars() {
                    // If the item is already in the hashset, we add the priority of the item to the total sum and break the loop
                    if all_items.contains_key(&item) {
                        total_sum += get_item_priority(item);
                        break;
                    }
                }
            }
        }
    }
    println!("Total sum: {}", total_sum);
}

fn part_two() {
    let mut total_sum = 0;
    let mut line_iterator = 0;
    let mut items = HashSet::new();
    let mut common_items = HashSet::new();
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(rucksack) = line {
                match line_iterator {
                    // On first line, insert all elements in the hashset items
                    0 => {
                        for item in rucksack.chars() {
                            items.insert(item);
                        }
                    }
                    // On second line, insert all common elements between items and line items in the hashset common_items
                    1 => {
                        for item in rucksack.chars() {
                            if items.contains(&item) {
                                common_items.insert(item);
                            }
                        }
                    }
                    // On the third line, check if the line item is in the hashset common_items, 
                    // if so, add the priority of the item to the total sum. Then reset the hashsets and the line iterator
                    2 => {
                        for item in rucksack.chars() {
                            if common_items.contains(&item) {
                                total_sum += get_item_priority(item);
                                break;
                            }
                        }
                        line_iterator = -1;
                        items.clear();
                        common_items.clear();                        
                    }
                    _ => {}
                }
                line_iterator += 1;
            }
        }
    }
    println!("Total sum: {}", total_sum);
}
