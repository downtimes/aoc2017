use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::cmp::max;

type Register = String;

#[derive(Debug, Clone)]
enum Condition {
    LE{reg: Register, value: i32},
    GR{reg: Register, value: i32},
    GQ{reg: Register, value: i32},
    EQ{reg: Register, value: i32},
    LQ{reg: Register, value: i32},
    UQ{reg: Register, value: i32}
}

#[derive(Debug)]
enum Instruction {
    INC{reg: Register, value: i32, condition: Condition},
    DEC{reg: Register, value: i32, condition: Condition}
}

impl Instruction {
    fn get_condition(&self) -> Condition {
        match self {
            &Instruction::INC{ref condition, ..} => condition.clone(),
            &Instruction::DEC{ref condition, ..} => condition.clone(),
        }
    }
}

fn parse_input(path: &Path) -> Vec<Instruction> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines = contents.split("\n").filter(|line| line != &"");
    let mut res = vec![];
    for line in lines {
        res.push(parse_line(line));
    }
    res
}

fn parse_line(input: &str) -> Instruction {
    let items: Vec<_> = input.split_whitespace().collect();
    let register = items[0].to_owned();
    let value = items[2].parse::<i32>().unwrap();
    let condition_register = items[4].to_owned();
    let condition_value = items[6].parse::<i32>().unwrap();
    let condition = match items[5] {
        ">" => Condition::GR {
            reg: condition_register,
            value: condition_value,
        },
        "<" => Condition::LE {
            reg: condition_register,
            value: condition_value,
        },
        "<=" => Condition::LQ {
            reg: condition_register,
            value: condition_value,
        },
        ">=" => Condition::GQ {
            reg: condition_register,
            value: condition_value,
        },
        "==" => Condition::EQ {
            reg: condition_register,
            value: condition_value,
        },
        "!=" => Condition::UQ {
            reg: condition_register,
            value: condition_value,
        },
        _ => panic!("Invalid input found with condition that we can not represent"),
    };
    let res = match items[1] {
        "inc" => Instruction::INC {
            reg: register,
            value: value,
            condition: condition,
        },
        "dec" => Instruction::DEC {
            reg: register,
            value: value,
            condition: condition,
        },
        _ => panic!("Invalid input found with instruction not recognized"),
    };

    res
}

fn main() {
    let program = parse_input(Path::new("./input"));
    //We use a hashmap instead of a fixed size thing because we have no idea how many registers there
    //actually are and what their names might even be!
    let mut registermap = HashMap::new();

    //For part two we also after every step check the highest value in our registermap
    let mut highest_value = 0;

    for instr in program.into_iter() {
        process(instr, &mut registermap);
        highest_value = max(highest_value, *registermap.values().max().unwrap());
    }
    println!("The highest register value after is {}", registermap.values().max().unwrap());
    println!("The highest register value always is {}", highest_value);
}

//Probably better to implement this function directly on the enum type
fn process(instr: Instruction, state: &mut HashMap<String, i32>)  {
    if condition_satisfied(instr.get_condition(), state) {
        match instr {
            Instruction::DEC{reg, value, ..} => {
                let ent = state.entry(reg.clone()).or_insert(0);
                *ent -= value;
            },
            Instruction::INC{reg, value, ..} => {
                let ent = state.entry(reg.clone()).or_insert(0);
                *ent += value;
            }
        }
    }
}


//Probably better to implement this function directly on the enum?
fn condition_satisfied(cond: Condition, state: &mut HashMap<String, i32>) -> bool{
    match cond {
        Condition::EQ{reg, value} => {
           let ent = state.entry(reg).or_insert(0);
           *ent == value
        },
        Condition::GQ{reg, value} => {
           let ent = state.entry(reg).or_insert(0);
           *ent >= value
        },
        Condition::GR{reg, value} => {
           let ent = state.entry(reg).or_insert(0);
           *ent > value

        },
        Condition::LE{reg, value} => {
           let ent = state.entry(reg).or_insert(0);
           *ent < value

        }, 
        Condition::LQ{reg, value} => {
           let ent = state.entry(reg).or_insert(0);
           *ent <= value

        },
        Condition::UQ{reg, value} => {
           let ent = state.entry(reg).or_insert(0);
           *ent != value
        }
    }
}