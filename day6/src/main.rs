use std::fs::File;
use std::io::Read;
use std::collections::HashSet;
use std::collections::HashMap;

fn read_input() -> [u64; 16] {
    let mut file = File::open("./input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let vec: Vec<u64> = contents
        .split_whitespace()
        .map(|item| item.parse::<u64>().unwrap())
        .collect();
    let mut res = [0; 16];
    for (ind, i) in res.iter_mut().enumerate() {
        *i = vec[ind];
    }
    res
}

const ARRAY_SIZE: usize = 16;

fn redistribute_step(input: [u64; ARRAY_SIZE]) -> [u64; ARRAY_SIZE] {
    let mut res = input;

    let red_ammount = input.iter().max().unwrap();
    let mut red_index = input.iter().position(|x| x == red_ammount).unwrap();
    res[red_index] = 0;

    //slow redistribution routine. With big numbers of blocks this is the bottleneck
    //Possible sheme: add num % ARRAY_SIZE to everyone and than only loop over the last
    //num / ARRAY_SIZE elements for the last distribution 
    for _ in 0..*red_ammount {
        red_index += 1;
        if red_index > ARRAY_SIZE - 1 {
            red_index = 0;
        }
        res[red_index] += 1;
    }
    res
}

fn main() {
    //Part1
    let mut memory = read_input();
    let mut prev_results = HashSet::new();
    let mut step_count = 0;
    while !prev_results.contains(&memory) {
        prev_results.insert(memory);
        memory = redistribute_step(memory);
        step_count += 1;
    }
    println!("Number of redistribution steps needed: {}", step_count);

    //Part2
    let mut memory = read_input();
    let mut step_count = 0;
    let mut prev_results = HashMap::new();
    while !prev_results.contains_key(&memory) {
        prev_results.insert(memory, step_count);
        step_count += 1;
        memory = redistribute_step(memory);
    }
    println!("Number of cycles between reoccurence: {}", step_count - prev_results.get(&memory).unwrap());
}


#[cfg(test)]
mod test {
    use super::*;

    fn test_red() {
        assert_eq!([2, 4, 1, 2], redistribute_step([0, 2, 7, 0]));
        assert_eq!([3, 1, 2, 3], redistribute_step([2, 4, 1, 7]));
        assert_eq!([0, 2, 3, 4], redistribute_step([3, 1, 2, 3]));
        assert_eq!([1, 3, 4, 1], redistribute_step([0, 2, 3, 4]));
        assert_eq!([2, 4, 1, 2], redistribute_step([1, 3, 4, 1]));
    }
}
