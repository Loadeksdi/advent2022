use std::io::{BufReader, Lines};
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};
use trees::{Tree, TreeWalk, Node};

#[derive(PartialEq)]
enum NodeType { Dir, File }
struct Value {
    value: u64,
    name: String,
    node_type: NodeType,
}

impl Value {
    fn new(value: u64, name: String, node_type: NodeType) -> Value {
        Value {
            value: value,
            name: name,
            node_type: node_type,
        }
    }
}
    
fn main() {
    let mut root = Tree::new(Value::new(0, "/".to_string(), NodeType::Dir));
    let mut walk = TreeWalk::from( root );
    if let Ok(lines) = read_lines("input.txt") {
        let mut copy = read_lines("input.txt").unwrap();
        for line in lines {
            if let Ok(term_line) = line {
                if term_line.starts_with("$") {
                    handle_command(&term_line, &walk, &mut copy);
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

fn handle_command(
    command: &str,
    tree: &TreeWalk<Value>,
    lines: &mut Lines<BufReader<File>>,
) {
    let args: Vec<&str> = command.split(" ").collect();
    match args[1] {
        "cd" => change_directory(tree, args[2]),
        "ls" => list_directory(tree, lines),
        _ => println!("Unknown command"),
    }
}

fn change_directory(
    tree: &TreeWalk<Value>,
    arg: &str,
) {
    match arg {
        ".." => {
            tree.to_parent();
        }
        "/" => {
            while tree.to_parent().is_some() {
                tree.to_parent();
            }            
        }
        _ => {
            if tree.get().unwrap().node().has_no_child(){
                return;
            }
            tree.to_child(0);
            /*
            let mut index = 0;
            while tree.get().is_some() {
                if tree.get().unwrap().node().data().name == arg {
                    tree.to_child(index);
                    return;
                } else {
                    tree.next();
                    index += 1;
                }
            }*/
        }
    }
}

fn list_directory(tree: &TreeWalk<Value>, lines: &mut Lines<BufReader<File>>) {
    let line = lines.next().unwrap().unwrap();
    let binding = tree.get().unwrap();
    let mut node = binding.node();
    while !line.starts_with("$") {
        if line.starts_with("dir") {
            let dir_name = line.split(" ").collect::<Vec<&str>>()[1];
            add_child(&mut node, 0, dir_name.to_string(), NodeType::Dir);
        } else {
            let file = line.split(" ").collect::<Vec<&str>>();
            add_child(&mut node, file[0].parse().unwrap(), file[1].to_string(), NodeType::File);
        }
    }
}

fn add_child(tree: &mut Node<Value>, value: u64, name: String, node_type: NodeType) {
    tree.push_back(Tree::new(Value::new(value, name, node_type)));
}