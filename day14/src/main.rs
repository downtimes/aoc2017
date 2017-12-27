use std::iter::FromIterator;
use std::collections::HashSet;

//was to lazy to implement my own data structure so I just used one from cargo
//Didn't reuse the dfs implementation from day12
extern crate disjoint_set;
use disjoint_set::DisjointSet;

#[allow(dead_code)]
const TEST_INPUT: &str = "flqrgnkx";
const GRID_ROW_COUNT: usize = 128;

//NOTE: There are many approaches to do Part2. I chose a pretty verbose
//one. I particularily liked this solution: 
//https://www.reddit.com/r/adventofcode/comments/7jpelc/2017_day_14_solutions/dr87cn7/
fn main() {
    //Part1
    let input = "ffayrhll";
    //let test_inp = "flqrgnkx";
    let sum: usize = (0..GRID_ROW_COUNT).map(|row| {
        let row_in = format!("{}-{}", input, row);
        let row_hash = knot_hash(&row_in);
        let row_bin = convert_hash_to_bits(&row_hash);
        row_bin.iter().filter(|&bit| *bit).count()
    }).sum();
    println!("The number of used bits in the grid is: {}", sum);

    //Part2 actually building the grid and doing union-find with it
    let input = "ffayrhll";
    let grid: Vec<Vec<bool>> = (0..GRID_ROW_COUNT).map(|row| {
        let row_in = format!("{}-{}", input, row);
        let row_hash = knot_hash(&row_in);
        let row_bin = convert_hash_to_bits(&row_hash);
        row_bin.into_iter().collect()
    }).collect();


    //Run through the whole grid once to build up the connected parts
    let mut uf = DisjointSet::new();
    for y in 0..GRID_ROW_COUNT {
        let column = &grid[y];
        for x in 0..column.len() {
            let elem = column[x];
            if elem {
                uf.make_set((y, x));
                for (ny, nx) in get_neighbour_idxs((y, x)) {
                    if grid[ny][nx] {
                        //Try to union them. If the neighbour is not yet in the
                        //data structure this will error. We don't care though
                        //since we will never hit that case
                        uf.union((y, x), (ny, nx)).unwrap();
                    }
                }
            }
        }
    }


    //Run through the uf once more to get the data out
    //NOTE: Has no convenient way to query the number of connected parts..
    let mut set = HashSet::new();
    for y in 0..GRID_ROW_COUNT {
        for x in 0..grid[y].len() {
            if let Some(tag) = uf.find((y, x)) {
                set.insert(tag);
            }
        }
    }

    println!("Number of connected areas in the grid: {}", set.len());
}

fn get_neighbour_idxs((y, x): (usize, usize)) -> Vec<(usize, usize)> {
    let mut res = vec![];
    //Give all indices while being carefull not to step outside the grid
    //We only have to consider neighbours on the top left from our current 
    //cause the others will be later in the algorithm and are not yet in our
    //union find data structure
    if y != 0 {
        res.push((y - 1, x));
    }
    if x != 0 {
        res.push((y, x - 1));
    }
    res
}

fn convert_hash_to_bits(hash: &str) -> Vec<bool> {
    let bin_iter = hash.chars().map(|c| {
        //convert the hex symbol to a number
        let num = c.to_digit(16).unwrap();
        //convert it back to binary
        format!("{:04b}", num)
    });
    let mut res = vec![];
    for bin_num in bin_iter {
        for bit in bin_num.chars() {
            if bit == '1' { 
                res.push(true) 
            } else { 
                res.push(false)
            }
        }
    }
    debug_assert!(res.len() == 4*hash.len());
    res
}


//Everything below here is hoisted from day10 solution for knot_hashes
const APPEND_LENGHTS: [u8; 5] = [17, 31, 73, 47, 23];
const ROPE_LENGTH: u8 = 255;
const ROUNDS: usize = 64;
fn knot_hash(inp: &str) -> String {
    let mut input = str_to_bytes(inp);
    input.extend_from_slice(&APPEND_LENGHTS);
    let mut rope = Vec::from_iter((0..ROPE_LENGTH));
    //stupid but range doesn't accept 256 because out of range of u8!
    rope.push(ROPE_LENGTH);
    //Do the same stuff for 64 rounds
    let mut position = 0;
    let mut skip = 0;
    for _ in 0..ROUNDS {
        for inp in input.iter() {
            reverse_subrope(position, position + *inp as usize, &mut rope);
            position = (position + *inp as usize + skip) % rope.len();
            skip += 1;
        }
    }
    let dense_hash = create_dense(&rope);
    dense_hash
        .iter()
        .map(|elem| format!("{:02x}", *elem))
        .collect()
}

fn str_to_bytes(inp: &str) -> Vec<u8> {
    inp.trim()
        .chars()
        .map(|c| {
            if (c as u32) < 256 {
                c as u8
            } else {
                panic!{"Only expecting ascii symbols"}
            }
        })
        .collect()
}

/*
    Reverse the subrope delimited by start and end
    end has to be > start but is not limited to the rope and instead
    wraps around
    start and end inclusive
*/
fn reverse_subrope(start: usize, end: usize, rope: &mut [u8]) {
    assert!(end >= start);
    assert!((end - start) <= rope.len());
    let mut start = start;
    let mut end = end - 1;
    while start < end {
        let len = rope.len();
        rope.swap(start % len, end % len);
        start += 1;
        end -= 1;
    }
}


fn create_dense(sparse: &[u8]) -> Vec<u8> {
    //move over our input in chunks of 16 items and calculate their XOR
    sparse
        .chunks(16)
        .map(|chunk| {
            chunk.iter().fold(0, |acc, &elem| acc ^ elem)
        })
        .collect()
}