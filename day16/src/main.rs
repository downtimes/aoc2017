use std::io::{self, Read};
use std::collections::HashMap;

#[derive(Debug)]
enum Instruction {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

//PART2 constants
const NUMBER_OF_DANCES: usize = 1_000_000_000;

fn main() {
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).unwrap();
    let instructions = parse(&input_str);
    let mut dancers = "abcdefghijklmnop".chars().collect::<Vec<_>>();   
    //PART1
    for inst in instructions.iter() {
        match *inst {
            Instruction::Spin(length) => spin(length, &mut dancers[..]),
            Instruction::Exchange(pos1, pos2) => dancers.swap(pos1, pos2),
            Instruction::Partner(a, b) => partner(a, b, &mut dancers[..]),
        }
    }
    println!("Dancers: {}", dancers.iter().collect::<String>());

    //Better solution to the problem here
    //https://www.reddit.com/r/adventofcode/comments/7k572l/2017_day_16_solutions/drbplq0/
    //It doesn't rely on there being any cycles and is mathematically sound
    //We rely on https://en.wikipedia.org/wiki/Landau%27s_function
    //as explained here https://www.reddit.com/r/adventofcode/comments/7k5th 
    //wich gives us an upper bound for cycle lenght of of 140^2 = 19600
    //PART2
    let mut seen = HashMap::new();
    let mut dancers = "abcdefghijklmnop".chars().collect::<Vec<_>>();   
    for i in 1..NUMBER_OF_DANCES + 1 {
        for inst in instructions.iter() {
            match *inst {
                Instruction::Spin(length) => spin(length, &mut dancers[..]),
                Instruction::Exchange(pos1, pos2) => dancers.swap(pos1, pos2),
                Instruction::Partner(a, b) => partner(a, b, &mut dancers[..]),
            }
        }
        //We have seen the same thing in a previous iteration
        if seen.contains_key(&dancers) {
            //Wait until we see the correct one to return again
            let period = i - seen[&dancers]; // The period length is since we last saw it
            let still_to_do = NUMBER_OF_DANCES - i;
            //If the rest we have still to do matches our period we have found
            //right configuration of dancers and can stop doing work
            if still_to_do % period == 0 {
                break;
            }
        }
        seen.insert(dancers.clone(), i);
    }
    println!("Dancers: {}", dancers.iter().collect::<String>());
}


//Stop doing everything with iterators. Hard to read and not very fast to program
//either. Use sparsely in the parse function
fn parse(inp: &str) -> Vec<Instruction> {
    inp.lines()
        .take(1)
        .map(|inp| {
            inp.split(",")
                .map(|item| {
                    let (ins, rest) = item.split_at(1);
                    match ins.chars().next().unwrap() {
                        's' => Instruction::Spin(rest.parse::<usize>().unwrap()),
                        'p' => {
                            let mut rest_iter = rest.chars();
                            let a = rest_iter.next().unwrap();
                            rest_iter.next(); //Jump over /
                            let b = rest_iter.next().unwrap();
                            Instruction::Partner(a, b)
                        }
                        'x' => {
                            let mut numbers = rest.split('/');
                            let pos1 = numbers.next().unwrap();
                            let pos2 = numbers.next().unwrap();
                            let pos1 = pos1.parse().unwrap();
                            let pos2 = pos2.parse().unwrap();
                            Instruction::Exchange(pos1, pos2)
                        }
                        _ => panic!("Wrong input format"),
                    }
                })
                .collect()
        })
        .next()
        .unwrap()
}

//In place rotation (right) of our array with 3 reverse operations
//Rust has a rotate function but it's nightly api only
fn spin(size: usize, input: &mut [char]) {
    let len = input.len();
    input.reverse();
    input[0..size].reverse();
    input[size..len].reverse();
}

fn partner(a: char, b: char, input: &mut [char]) {
    let a_idx = input.iter().position(|&x| x == a).unwrap();
    let b_idx = input.iter().position(|&x| x == b).unwrap();
    input.swap(a_idx, b_idx);
}
