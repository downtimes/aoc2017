use std::io::{self, Read};
use std::collections::HashSet;

#[derive(Debug)]
enum Facing {
    N,
    S,
    W,
    E,
}

#[derive(Debug)]
struct AdvancedVirus {
    pos: (i32, i32),
    dir: Facing,
    infected: u32,
    weakend: HashSet<(i32, i32)>,
    flagged: HashSet<(i32, i32)>,
}

impl AdvancedVirus {
    fn step(&mut self, map: &mut HashSet<(i32, i32)>) {
        use Facing::*;
        if !map.contains(&self.pos) && !self.weakend.contains(&self.pos) {
            self.weakend.insert(self.pos);
            match self.dir {
                N => { self.dir = W }
                S => { self.dir = E }
                E => { self.dir = N }
                W => { self.dir = S }
            }
        } else if self.weakend.contains(&self.pos) {
            self.weakend.remove(&self.pos);
            map.insert(self.pos);
            self.infected += 1;
        } else if !self.flagged.contains(&self.pos) && map.contains(&self.pos) {
            self.flagged.insert(self.pos);
             match self.dir {
                N => { self.dir = E }
                S => { self.dir = W }
                E => { self.dir = S }
                W => { self.dir = N }
            }
        } else {
            self.flagged.remove(&self.pos);
            map.remove(&self.pos);
            match self.dir {
                N => { self.dir = S }
                S => { self.dir = N }
                E => { self.dir = W }
                W => { self.dir = E }
            }
        }
        self.move_me();
    }
    
    fn move_me(&mut self) {
        use Facing::*;
        match self.dir {
            N => self.pos = (self.pos.0, self.pos.1 - 1),
            E => self.pos = (self.pos.0 + 1, self.pos.1),
            W => self.pos = (self.pos.0 - 1, self.pos.1),
            S => self.pos = (self.pos.0, self.pos.1 + 1),
        }
    }
}

#[derive(Debug)]
struct Virus {
    pos: (i32, i32),
    dir: Facing,
    infected: u32,
}

impl Virus {
    fn step(&mut self, map: &mut HashSet<(i32, i32)>) {
        use Facing::*;
        if map.contains(&self.pos) {
            map.remove(&self.pos);
            match self.dir {
                N => { self.dir = E }
                S => { self.dir = W }
                E => { self.dir = S }
                W => { self.dir = N }
            }
        } else {
            map.insert(self.pos);
            self.infected += 1;
            match self.dir {
                N => { self.dir = W }
                S => { self.dir = E }
                E => { self.dir = N }
                W => { self.dir = S }
            }
        }
        self.move_me();
    }

    fn move_me(&mut self) {
        use Facing::*;
        match self.dir {
            N => self.pos = (self.pos.0, self.pos.1 - 1),
            E => self.pos = (self.pos.0 + 1, self.pos.1),
            W => self.pos = (self.pos.0 - 1, self.pos.1),
            S => self.pos = (self.pos.0, self.pos.1 + 1),
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap(); 
    let infect_map = parse(&input);
    let mut map1 = infect_map.clone();
    let mut virus = Virus {
        pos: (0, 0),
        dir: Facing::N,
        infected: 0,
    };
    for _ in 0..10_000 {
        virus.step(&mut map1);
    }
    println!("{}", virus.infected);
    
    let mut advanced_virus = AdvancedVirus {
        pos: (0, 0),
        dir: Facing::N,
        infected: 0,
        weakend: HashSet::new(),
        flagged: HashSet::new(),
    };
    let mut map2 = infect_map.clone();
    for _ in 0..10_000_000 {
        advanced_virus.step(&mut map2);
    }
    println!("{}", advanced_virus.infected);
}

fn parse(inp: &str) -> HashSet<(i32, i32)> {
    let lines = inp.trim().lines().collect::<Vec<_>>();
    let height = lines.len() as i32;
    let width = lines[0].len() as i32;
    let mut res = HashSet::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                res.insert((x as i32 - width/2, y as i32 - height/2));
            }
        }
    }
    res
}
