use std::io::{self, Read};
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = parse(&input);
    let mut machine = Machine {
        regs: HashMap::new(),
        pc: 0,
        mul_count: 0,
    };
    machine.run(&instructions);
    println!("number of mul: {}", machine.mul_count);
    //Part 2
    //see reverse engineered solution in transcript
    //it counts all non-prime between 107900 and 124900 in step 17
    let mut b = 107900;
    let mut count = 0;
    while b <= 124900 {
        if !is_prime(b) {
            count += 1;
        }
        b += 17;
    }
    println!("number of non-primes: {}", count);
}

fn is_prime(num: u32) -> bool {
    if num <= 1 { return false; }
    if num <= 3 { return true; }
    if ((num % 2) == 0) || ((num % 3) == 0) {
        return false;
    }
    let mut i = 5;
    while (i * i) <= num {
        if ((num % i) == 0) || ((num % (i + 2)) == 0) {
            return false;
        }
        i += 6;
    }
    true
}


//Everything below here is hoistet from day18 and slightly modified
#[derive(Debug, Clone, Copy)]
struct Ridx(char);


#[derive(Debug, Clone, Copy)]
enum Operand {
    Register(Ridx),
    Constant(i64),
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Set(Ridx, Operand),
    Sub(Ridx, Operand),
    Mul(Ridx, Operand),
    Jnz(Operand, Operand),
}

#[derive(Debug)]
struct Machine {
    regs: HashMap<char, i64>,
    pc: isize,
    mul_count: u64,
}

impl Machine {
    fn step(&mut self, inst: Instruction) {
        self.pc += 1;
        use Instruction::*;
        match inst {
            Set(Ridx(r), operand) => { 
                let val = self.reg_or_con(operand);
                *self.regs.entry(r).or_insert(0) = val;
            }
            Mul(Ridx(r), operand) => { 
                self.mul_count += 1;
                let val = self.reg_or_con(operand);
                *self.regs.entry(r).or_insert(0) *= val;
            }
            Sub(Ridx(r), operand) => {
                let val = self.reg_or_con(operand);
                *self.regs.entry(r).or_insert(0) -= val;
            }
            Jnz(operand1, operand2) => { 
                let val = self.reg_or_con(operand2);
                let con = self.reg_or_con(operand1);
                if con != 0 {
                    self.pc -= 1;
                    self.pc += val as isize;
                }
            }
        }
    }

    fn reg_or_con(&mut self, op: Operand) -> i64 {
        use Operand::*;
        match op {
            Register(Ridx(idx)) => {
                *self.regs.entry(idx).or_insert(0)
            }
            Constant(con) => con,
        }
    }

    fn run(&mut self, instr: &Vec<Instruction>) {
        loop {
            if self.pc < 0 || self.pc >= instr.len() as isize { break }
            let pc = self.pc;
            self.step(instr[pc as usize]);
        }
    }
}

fn parse_reg(val: Option<&str>) -> Ridx {
    let val = val.unwrap();
    Ridx(val.chars().next().unwrap())
}

fn parse_op(val: Option<&str>) -> Operand {
    let val = val.unwrap();
    match val.chars().next().unwrap().is_alphabetic() {
        true => Operand::Register(Ridx(val.chars().next().unwrap())),
        false => Operand::Constant(val.parse().unwrap()),

    }
}

fn parse(inp: &str) -> Vec<Instruction> {
    use Instruction::*;
    let lines: Vec<_> = inp.trim().split("\n").collect();
    let mut res = vec![];
    for line in lines {
        let (op, ops) = line.split_at(4);
        let mut ops = ops.trim().split_whitespace();
        let ins = match op.trim() {
           "set" => Set(parse_reg(ops.next()), parse_op(ops.next())), 
           "mul" => Mul(parse_reg(ops.next()), parse_op(ops.next())), 
           "sub" => Sub(parse_reg(ops.next()), parse_op(ops.next())),
           "jnz" => Jnz(parse_op(ops.next()), parse_op(ops.next())), 
           _ => panic!("Unknown Instruction")
        };
        res.push(ins);
    }
    res
}