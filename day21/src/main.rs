use std::io::{self, Read};
use std::collections::HashMap;

type Grid = Vec<Vec<bool>>;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let rules = parse(&input);
    let mut grid = vec![
        vec![false, true, false],
        vec![false, false, true],
        vec![true, true, true],
    ];
    for _ in 0..5 {
        grid = step(grid, &rules);
    }
    let true_count = grid.iter()
        .flat_map(|row| row.iter())
        .filter(|&&e| e)
        .count();
    println!("{:?}", true_count);

    //Part 2
    //do the other 13 iterations
    for _ in 0..13 {
        grid = step(grid, &rules);
    }
    let true_count = grid.iter()
        .flat_map(|row| row.iter())
        .filter(|&&e| e)
        .count();
    println!("{:?}", true_count);
}

fn step(grid: Grid, rules: &HashMap<Grid, Grid>) -> Grid {
    let len = grid.len();
    let step_size = if (len % 2) == 0 { 2 } else { 3 };
    let new_size = (len * (step_size + 1)) / step_size;
    let mut new = vec![vec![false; new_size]; new_size];
    for y in 0..len / step_size {
        for x in 0..len / step_size {
            let mut ref_grid = vec![];
            for y2 in y * step_size..y * step_size + step_size {
                ref_grid.push(
                    grid[y2][x * step_size..x * step_size + step_size].to_vec()
                );
            }
            let replace = rules.get(&ref_grid);
            let replace = replace.expect("No pattern found");
            let step_size = step_size + 1;
            for y2 in 0..step_size {
                for x2 in 0..step_size {
                    new[y * step_size + y2][x * step_size + x2] = replace[y2][x2];
                }
            }
        }
    }
    new
}

fn parse(inp: &str) -> HashMap<Grid, Grid> {
    let lines = inp.trim().lines().collect::<Vec<_>>();
    let mut rules = HashMap::new();
    for line in lines {
        let parts = line.split(" => ").collect::<Vec<_>>();
        let pattern = parts[0]
            .split('/')
            .map(|row| row.chars().map(|c| c == '#').collect())
            .collect();
        let output = parts[1]
            .split('/')
            .map(|row| row.chars().map(|c| c == '#').collect())
            .collect::<Vec<_>>();
        for pattern in create_patterns(pattern) {
            rules.insert(pattern, output.clone());
        }
    }
    rules
}

fn create_patterns(grid: Grid) -> Vec<Grid> {
    let mut grid = grid;
    let mut res = vec![grid.clone()];
    transpose(&mut grid);
    res.push(grid.clone());
    flip_v(&mut grid);
    res.push(grid.clone());
    transpose(&mut grid);
    res.push(grid.clone());
    flip_v(&mut grid);
    res.push(grid.clone());
    transpose(&mut grid);
    res.push(grid.clone());
    flip_v(&mut grid);
    res.push(grid.clone());
    transpose(&mut grid);
    res.push(grid);
    res
}

fn flip_v(grid: &mut Grid) {
    for line in grid {
        let mut start = 0;
        let mut end = line.len() - 1;
        while start < end {
            unsafe {
                std::ptr::swap(&mut line[start], &mut line[end]);
            }
            start += 1;
            end -= 1;
        }
    }
}

fn transpose(grid: &mut Grid) {
    for x in 1..grid[1].len() {
        for y in 0..x {
            unsafe {
                std::ptr::swap(&mut grid[y][x], &mut grid[x][y]);
            }
        }
    }
}
