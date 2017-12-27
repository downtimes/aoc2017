//use std::io::{self, Read};
use std::collections::VecDeque;

struct Tape {
    offset: usize,
    tape: VecDeque<bool>,
}

impl Tape {
    fn read(&mut self, pos: isize) -> bool {
        //TODO: check index is max one out
        if pos == -(self.offset as isize) || pos == (self.tape.len() as isize - self.offset as isize) {
            self.write(pos, false);
        }
        return self.tape[(self.offset as isize + pos) as usize]
    }

    fn write(&mut self, pos: isize, val: bool) {
        //TODO: check index is max one out
        if pos == -(self.offset as isize) {
            self.tape.push_front(val);
            self.offset += 1;
            return;
        } else if pos == (self.tape.len() as isize - self.offset as isize) {
            self.tape.push_back(val);
            return;
        }
        self.tape[(self.offset as isize + pos) as usize] = val;
    }

    fn checksum(&self) -> usize {
        self.tape.iter().filter(|&&b| b).count()
    }
}

#[derive(Hash, PartialEq, Eq)]
enum State {
    A,
    B,
    C,
    D,
    E,
    F
}

struct Head {
    pos: isize,
    state: State,
}

impl Head {
    fn step(&mut self, tape: &mut Tape) {
        let val = tape.read(self.pos);
        match self.state {
            State::A => {
                if !val {
                    tape.write(self.pos, true);
                    self.pos += 1;
                    self.state = State::B;
                } else {
                    tape.write(self.pos, false);
                    self.pos += 1;
                    self.state = State::C;
                }
            }
            State::B => {
                if !val {
                    self.pos -= 1;
                    self.state = State::A;
                } else {
                    tape.write(self.pos, false);
                    self.pos += 1;
                    self.state = State::D;
                }
            }
            State::C => {
                if !val {
                    tape.write(self.pos, true);
                    self.pos += 1;
                    self.state = State::D;
                } else {
                    self.pos += 1;
                    self.state = State::A;
                }
            }
            State::D => {
                if !val {
                    tape.write(self.pos, true);
                    self.pos -= 1;
                    self.state = State::E;
                } else {
                    tape.write(self.pos, false);
                    self.pos -= 1;
                    self.state = State::D;
                }
            }
            State::E => {
                if !val {
                    tape.write(self.pos, true);
                    self.pos += 1;
                    self.state = State::F;
                } else {
                    self.pos -= 1;
                    self.state = State::B;
                }
            }
            State::F => {
                if !val {
                    tape.write(self.pos, true);
                    self.pos += 1;
                    self.state = State::A;
                } else {
                    self.pos += 1;
                    self.state = State::E;
                }
            }
        }
    }
}

fn main() {
// No input gets parsed. The machine is handcoded
//    let mut input = String::new();
//    io::stdin().read_to_string(&mut input).unwrap();
    let mut head = Head {
        pos: 0,
        state: State::A,
    };
    let mut tape = Tape {
        offset: 0,
        tape: VecDeque::new(),
    };
    tape.tape.push_back(false);
    for _ in 0..12399302 {
        head.step(&mut tape);
    }
    println!("Number of 1s on tape: {}", tape.checksum());
}
