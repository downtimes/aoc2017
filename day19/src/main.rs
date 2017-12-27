use std::io::{self, Read};

#[derive(Debug)]
enum Direction {
    Up(usize, usize),
    Down(usize, usize),
    Left(usize, usize),
    Right(usize, usize),
}

type Grid = Vec<Vec<char>>;

impl Direction {
    fn step(&mut self, grid: &Grid) -> char {
        match self {
            &mut Direction::Up(ref x, ref mut y) => { *y -= 1; grid[*y][*x] }
            &mut Direction::Down(ref x, ref mut y) => { *y += 1; grid[*y][*x] }
            &mut Direction::Left(ref mut x, ref y) => { *x -= 1; grid[*y][*x] }
            &mut Direction::Right(ref mut x, ref y) => { *x += 1; grid[*y][*x] }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let grid = parse(&input);
    let mut packet = find_start(&grid);
    let mut counter = 1;
    let mut string = String::new();
    loop {
        let next = packet.step(&grid);
        match next {
            '+' => packet = change_direction(packet, &grid),
            a if a.is_alphabetic() => string.push(a),
            ' ' => break,
            _ => {}
        }
        counter += 1;
    }
    println!("The result for Part1: {}", string);
    println!("The result for Part2: {}", counter);
}

fn change_direction(dir: Direction, grid: &Grid) -> Direction {
    match dir {
        Direction::Down(x, y) | Direction::Up(x, y) => {
            if grid[y][x - 1] == '-' || grid[y][x - 1].is_alphabetic() {
                Direction::Left(x, y)
            } else {
                Direction::Right(x, y)
            }
        }
        Direction::Left(x, y) | Direction::Right(x, y) => {
            if grid[y - 1][x] == '|' || grid[y - 1][x].is_alphabetic() {
                Direction::Up(x, y)
            } else {
                Direction::Down(x, y)
            }
        }
    }
}

fn find_start(grid: &Grid) -> Direction {
    let x = grid[0].iter().position(|&c| c == '|').unwrap();
    Direction::Down(x, 0)
}

fn parse(inp: &str) -> Grid {
    inp.lines().filter(|line| line != &"").map(|line| {
        line.chars().collect()
    }).collect()
}