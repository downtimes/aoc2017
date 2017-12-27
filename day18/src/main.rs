use std::io::{self, Read};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

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
    Add(Ridx, Operand),
    Mul(Ridx, Operand),
    Mod(Ridx, Operand),
    Jgz(Operand, Operand),
    Snd(Ridx),
    Rcv(Ridx),
}

#[derive(Debug)]
struct Machine {
    regs: HashMap<char, i64>,
    pc: isize,
    last_played: i64,
    lock_tx: Sender<Report>,
    tx: Sender<i64>,
    rx: Receiver<i64>,
    id: usize,
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
            Add(Ridx(r), operand) => { 
                let val = self.reg_or_con(operand);
                *self.regs.entry(r).or_insert(0) += val;
            }
            Mul(Ridx(r), operand) => { 
                let val = self.reg_or_con(operand);
                *self.regs.entry(r).or_insert(0) *= val;
            }
            Mod(Ridx(r), operand) => { 
                let val = self.reg_or_con(operand);
                *self.regs.entry(r).or_insert(0) %= val;
            }
            Jgz(operand1, operand2) => { 
                let val = self.reg_or_con(operand2);
                let con = self.reg_or_con(operand1);
                if con > 0 {
                    self.pc -= 1;
                    self.pc += val as isize;
                }
            }
            Snd(Ridx(r)) => {
                self.tx.send(*self.regs.entry(r).or_insert(0));
                match self.id {
                    1 => {self.lock_tx.send(Report::P1Snd);}
                    0 => {}
                    _ => panic!("Unknown Threadid")
                };
            }
            Rcv(Ridx(r)) => {
                let val = self.rx.recv().unwrap();
                *self.regs.entry(r).or_insert(0) = val;
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

    fn run(&mut self, instr: Vec<Instruction>) {
        loop {
            if self.pc < 0 || self.pc >= instr.len() as isize { break }
            let pc = self.pc;
            self.step(instr[pc as usize]);
        }
    }
}
enum Report {
    P1Snd,
}
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let inp = parse(&input);
    let (lock_tx, lock_rx) = channel();
    let (p0_tx, p1_rx) = channel();
    let (p1_tx, p0_rx) = channel();
    let lock_tx2 = lock_tx.clone();
    let inp2 = inp.clone();
    //p0
    thread::spawn(move || {
        let mut machine = Machine {
            regs: HashMap::new(),
            pc: 0,
            last_played: 0,
            lock_tx: lock_tx,
            tx: p0_tx,
            rx: p0_rx,
            id: 0,
        };
        machine.run(inp);
    });
    //P1
    thread::spawn(move || {
        let mut machine = Machine {
            regs: HashMap::new(),
            pc: 0,
            last_played: 0,
            lock_tx: lock_tx2,
            tx: p1_tx,
            rx: p1_rx,
            id: 1,
        };
        machine.regs.insert('p', 1);
        machine.run(inp2);
    });

    let mut counter = 0;
    loop {
        match lock_rx.recv_timeout(Duration::from_millis(500)) {
            Err(_) => break,
            Ok(_) => {
                counter += 1;
            }
        }
    }
    println!("{}", counter);
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
           "snd" => Snd(parse_reg(ops.next())), 
           "add" => Add(parse_reg(ops.next()), parse_op(ops.next())),
           "set" => Set(parse_reg(ops.next()), parse_op(ops.next())), 
           "mul" => Mul(parse_reg(ops.next()), parse_op(ops.next())), 
           "mod" => Mod(parse_reg(ops.next()), parse_op(ops.next())), 
           "rcv" => Rcv(parse_reg(ops.next())), 
           "jgz" => Jgz(parse_op(ops.next()), parse_op(ops.next())), 
           _ => panic!("Unknown Instruction")
        };
        res.push(ins);
    }
    res
}