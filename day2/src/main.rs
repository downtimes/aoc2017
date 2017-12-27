extern crate itertools;

use std::io::Read;
use itertools::Itertools;

fn main() {
    let mut file = std::fs::File::open("./input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines = contents.split("\n");
    let checksum1: u32 = lines.clone().map(|line| calc_line_difference(line)).sum();
    let checksum2: u32 = lines.map(|line| calc_div(line)).sum();


    println!("The checksum of the lines is: {}", checksum1);
    println!("The ccheksum of the lines is: {}", checksum2);
}

fn calc_div(line: &str) -> u32 {
    let elements: Vec<i32> = line.split_whitespace()
        .map(|x| x.parse::<i32>().expect("Input was not an i32 number"))
        .collect::<Vec<i32>>();
    let combinations = elements.iter().cartesian_product(elements.iter());
    let divisibles = combinations
        .filter(|&(x, y)| (x != y) && (x % y) == 0)
        .next()
        .unwrap_or((&0, &1));

    let (x, y) = divisibles;
    (x / y) as u32
}

fn calc_line_difference(line: &str) -> u32 {
    let elements: Vec<i32> = line.split_whitespace()
        .map(|x| x.parse::<i32>().expect("Input was not an i32 number"))
        .collect();

    let diff = elements.iter().max().unwrap_or(&0) - elements.iter().min().unwrap_or(&0);
    diff as u32
}
