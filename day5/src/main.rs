use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    // Create 9 stacks according to the problem, could be done dynamically but this is easier
    let mut stacks: [Vec<char>; 9] = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(crate_line) = line {
                // Ignore unused lines after crates lines
                if crate_line.starts_with(" 1") || crate_line == "" {
                    continue;
                } else if crate_line.starts_with("move") {
                    // Parse move instructions
                    let instructions = crate_line
                        .split_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect::<Vec<usize>>();
                    let number_of_crate_to_move: usize = instructions[0];
                    let from_stack: usize = instructions[1];
                    let to_stack: usize = instructions[2];
                    // Get a slice of the crates to move and append it to the destination stack
                    let crates_slice = stacks[from_stack - 1]
                        .split_off(stacks[from_stack - 1].len() - number_of_crate_to_move);
                    stacks[to_stack - 1].append(&mut crates_slice.to_vec());
                } else {
                    let input_vec: Vec<&str> = crate_line.split(" ").collect();
                    let mut counter = 0;
                    let mut four = 0;
                    for single_crate in input_vec {
                        // If the crate is not empty, we add it to the stack
                        if single_crate != "" {
                            four = 0;
                            let stack = &mut stacks[counter];
                            // We ignore the "[" and "]" characters
                            stack.insert(0, single_crate.chars().nth(1).unwrap());
                            stacks[counter] = stack.to_vec();
                            counter += 1;
                        } else {
                            four += 1;
                            // Every 4 empty strings, we have to skip a stack
                            if four % 4 == 0 {
                                counter += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    // Print the code coming from the crate on the top of each stack
    let new_code = stacks.map(|stack| *stack.last().unwrap()).iter().collect::<String>();
    println!("{}", new_code);
}

// Read input file, taken from rust-by-example
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
