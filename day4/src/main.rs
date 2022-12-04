use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    //part_one();
    part_two();
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
    let mut overlapping_areas_count = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(area) = line {
                // Split input into two zones, then split each zone into begin and end
                let vec: Vec<&str> = area.split(",").collect();
                let zone_one: Vec<&str> = vec[0].split("-").collect();
                let zone_two: Vec<&str> = vec[1].split("-").collect();
                let zone_one_begin: i32 = zone_one[0].parse().unwrap();
                let zone_one_end: i32 = zone_one[1].parse().unwrap();
                let zone_two_begin: i32 = zone_two[0].parse().unwrap();
                let zone_two_end: i32 = zone_two[1].parse().unwrap();
                // Check if the two zones fully overlap, if so, increment the counter
                if (zone_one_begin >= zone_two_begin && zone_one_end <= zone_two_end)
                    || (zone_two_begin >= zone_one_begin && zone_two_end <= zone_one_end)
                {
                    overlapping_areas_count += 1;
                }
            }
        }
    }
    println!("Number of overlapping areas: {}", overlapping_areas_count);
}

fn part_two() {
    let mut overlapping_areas_count = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(area) = line {
                // Split input into two zones, then split each zone into begin and end
                let vec: Vec<&str> = area.split(",").collect();
                let zone_one: Vec<&str> = vec[0].split("-").collect();
                let zone_two: Vec<&str> = vec[1].split("-").collect();
                let zone_one_begin: i32 = zone_one[0].parse().unwrap();
                let zone_one_end: i32 = zone_one[1].parse().unwrap();
                let zone_two_begin: i32 = zone_two[0].parse().unwrap();
                let zone_two_end: i32 = zone_two[1].parse().unwrap();
                // Check if the two zones partially overlap, if so, increment the counter
                if (zone_one_begin >= zone_two_begin && zone_one_begin <= zone_two_end)
                    || (zone_one_end >= zone_two_begin && zone_one_end <= zone_two_end)
                    || (zone_two_begin >= zone_one_begin && zone_two_begin <= zone_one_end)
                    || (zone_two_end >= zone_one_begin && zone_two_end <= zone_one_end)
                {
                    overlapping_areas_count += 1;
                }
            }
        }
    }
    println!("Number of overlapping areas: {}", overlapping_areas_count);
}
