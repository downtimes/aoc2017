use std::io::{self, Read};
use std::ops::Add;
use std::collections::HashSet;

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Vector3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vector3 {
    fn distance(&self) -> u64 {
        self.x.abs() as u64 + self.y.abs() as u64 + self.z.abs() as u64
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Debug)]
struct Particle {
    p: Vector3,
    v: Vector3,
    a: Vector3,
}

impl Particle {
    fn step(&mut self) {
        self.v = self.v + self.a;
        self.p = self.p + self.v;
    }
}

//TODO: fix this wrong solution https://www.reddit.com/r/adventofcode/comments/7l1766/2017_day_20_part_2_so_stopping_conditions_anyone/driuj1f/
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut particles = parse(&input);
    //P1
    {
        let mut enumerated = particles.iter().enumerate().collect::<Vec<_>>();
        enumerated.sort_unstable_by(|&(_,p1), &(_,p2)| p2.p.cmp(&p1.p));
        enumerated.sort_by(|&(_,p1), &(_,p2)| p2.v.cmp(&p1.v));
        let closest = enumerated
            .iter()
            .min_by_key(|&&(_, p)| p.a.distance())
            .unwrap();
        println!("{}", closest.0);
    }
    //P2
    //Bruteforce attempt
    for _ in 0..100_000 {
        let mut to_test = HashSet::new();
        let mut to_remove = HashSet::new();
        for i in 0..particles.len() {
            if to_test.contains(&particles[i].p) {
                to_remove.insert(particles[i].p);
            } else {
                to_test.insert(particles[i].p);
            }
        }
        particles.retain(|e| !to_remove.contains(&e.p));
        particles.iter_mut().for_each(|p| p.step());
    }
    println!("{}", particles.len());
}

fn parse(inp: &str) -> Vec<Particle> {
    let lines = inp.trim().lines().collect::<Vec<_>>();
    let mut res = vec![];
    for line in lines {
        let mut comp = line.split(", ").collect::<Vec<_>>();
        for com in comp.iter_mut() {
            let end = com.len() - 1;
            *com = com.get(3..end).unwrap();
        }
        let mut pos = comp[0].trim().split(",");
        let mut vel = comp[1].trim().split(",");
        let mut acc = comp[2].trim().split(",");
        let p = Vector3 {
            x: pos.next().unwrap().parse().unwrap(),
            y: pos.next().unwrap().parse().unwrap(),
            z: pos.next().unwrap().parse().unwrap(),
        };
        let v = Vector3 {
            x: vel.next().unwrap().parse().unwrap(),
            y: vel.next().unwrap().parse().unwrap(),
            z: vel.next().unwrap().parse().unwrap(),
        };
        let a = Vector3 {
            x: acc.next().unwrap().parse().unwrap(),
            y: acc.next().unwrap().parse().unwrap(),
            z: acc.next().unwrap().parse().unwrap(),
        };
        let particle = Particle { p, v, a };
        res.push(particle);
    }
    res
}
