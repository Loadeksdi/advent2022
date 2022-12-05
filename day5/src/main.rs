use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
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
                if crate_line.starts_with(" 1") || crate_line == "" {
                    continue;
                } else if crate_line.starts_with("move") {
                    let instructions: Vec<usize> = crate_line
                        .chars()
                        .filter(|c| c.is_digit(10))
                        .map(|c| c.to_digit(10).unwrap() as usize)
                        .collect();
                    let number_of_crate_to_move = instructions[0];
                    let from_stack = instructions[1];
                    let to_stack = instructions[2];
                    for _i in 0..(number_of_crate_to_move) {
                        let crate_in_stack = stacks[from_stack - 1].pop();
                        println!(
                            "Moving crate {:?} from stack {:?} to stack {:?}",
                            crate_in_stack,
                            from_stack,
                            to_stack
                        );
                        if crate_in_stack.is_none() {
                            continue;
                        }                        
                        stacks[to_stack - 1].push(crate_in_stack.unwrap());
                    }
                } else {
                    let vec: Vec<&str> = crate_line.split(" ").collect();
                    let mut counter = 0;
                    let mut empty = false;
                    for ele in vec {
                        if ele != "" {
                            let stack = &mut stacks[counter];
                            stack.insert(0, ele.chars().nth(1).unwrap());
                            stacks[counter] = stack.to_vec();
                            counter += 1;
                        } else {
                            if !empty {
                                counter += 1;
                            }
                            empty = true;
                        }
                    }
                }
            }
        }
    }
    for i in 0..stacks.len() {
        println!("Stack {}: {:?}", i, stacks.get(i).unwrap().last());
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
