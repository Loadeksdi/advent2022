use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};
use trees::{tr, Tree}; // tr stands for tree

struct Node {
    name: String,
    value: u64,
}
fn main() {
    let mut root: Tree<Node> = tr(Node {
        name: "/".to_string(),
        value: 0,
    });
    let mut cur = &root;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(term_line) = line {
                if term_line == "cd /" {
                    cur = &root;
                } else if term_line == "cd .." {
                    cur = cur.parent().unwrap();
                } else if term_line.starts_with("ls") {
                    tree.list();
                }
            }
        }
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
