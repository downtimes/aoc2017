use std::io::{self, Read};

const MOD_NUMBER: u64 = 2147483647;
const A_FACTOR: u64 = 16807;
const B_FACTOR: u64 = 48271;

const AMOUNT_WANTED: usize = 40_000_000;
const AMOUNT_WANTED_P2: usize = 5_000_000;

#[derive(Debug)]
struct Generator {
    factor: u64,
    current: u64,
    divisible: u64,
}

impl Iterator for Generator {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        loop {
            self.current *= self.factor; 
            self.current %= MOD_NUMBER;
            if self.current % self.divisible == 0 {
                break;
            }
        }
        Some(self.current)
    }
}

fn main() {
    //From now on let's get the input from the stdin instead of including the
    //file. Easier for testcases which we can then just pipe in the program
    //Instead of recompiling.
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).unwrap();
    //Part1
    let (gena, genb) = parse(&input_str, Part::P1);
    let sum = gena.zip(genb).take(AMOUNT_WANTED)
        //Trim the numbers to 16bit
        .map(|(a, b)| {
            (a & 0xFFFF, b & 0xFFFF)
        //Fiter for identical numbers and count them
        }).filter(|&(ref a, ref b)| {
            a == b
        }).count();
    println!("The number of numbers with lower 16 bit matching is: {}", sum);

    //Part2
    let (gena, genb) = parse(&input_str, Part::P2);
    let sum = gena.zip(genb).take(AMOUNT_WANTED_P2)
        //Trim the numbers to 16bit
        .map(|(a, b)| {
            (a & 0xFFFF, b & 0xFFFF)
        //Fiter for identical numbers and count them
        }).filter(|&(ref a, ref b)| {
            a == b
        }).count();
    println!("The number of numbers with lower 16 bit matching is: {}", sum);
}

#[derive(PartialEq, Eq)]
enum Part {
    P1,
    P2,
}

fn parse(inp: &str, part: Part) -> (Generator, Generator) {
    let mut lines = inp.lines();
    let a_start = lines.next().unwrap().split_whitespace().nth(4).unwrap().parse().unwrap();
    let b_start = lines.next().unwrap().split_whitespace().nth(4).unwrap().parse().unwrap();
    
    let a = Generator {
        factor: A_FACTOR,
        current: a_start,
        divisible: if part == Part::P1 { 1 } else { 4 }
    };
    let b = Generator {
        factor: B_FACTOR,
        current: b_start,
        divisible: if part == Part::P1 { 1 } else { 8 }
    };

    (a, b)
}
