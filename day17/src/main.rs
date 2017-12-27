use std::io::{self, Read};

const NUM_CYCLES: usize = 2017;
const NUM_CYCLES2: usize = 50_000_000;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let step_count = parse(&input);

    println!("Answer to P1: {}", get_part1(step_count as usize));
    println!("Answer to P1: {}", get_part2(step_count as usize));
}


//We are only interested in the last element inserted at position 1
//Therefore we don't actually compute the vector and just simulate it 
fn get_part2(step: usize) -> usize {
    let mut current_pos = 0;
    let mut last = 0;

    for i in 0..NUM_CYCLES2 {
        //The vector in step i would be i+1 in size
        current_pos = (current_pos + step) % (i + 1);
        //if we are in the position we are searching for update our value
        if current_pos + 1 == 1 {
            last = i + 1;
        }
        //don't forget to update our position because we "inserted" the item
        current_pos += 1;
    }
    return last
}

//Rotates right and left, -size means left; standard is right
fn rotate(size: isize, input: &mut [u32]) {
    if size == 0 { return }
    if size.abs() as usize == input.len() { return }
    let len = input.len();
    input.reverse();
    let (rot1, rot2) = if size < 0 {
        input.split_at_mut((len as isize + (size % len as isize)) as usize)
    } else {
        input.split_at_mut(size as usize % len)
    };
    rot1.reverse();
    rot2.reverse();
}

//NOTE: instead of what I wrote (see comments) this is a classical case for
//rotate. Always rotate the current position to the end by shifting the array
//step to the left. So we can only use appends to the end of the vector! 
fn get_part1(step: usize) -> u32 {
    let mut buf = Vec::with_capacity(NUM_CYCLES + 2);
    buf.push(0);
    hello

    //let mut current_pos = 0;
    for i in 0..NUM_CYCLES {
        rotate(-(step as isize), &mut buf[..]);
        //current_pos = (current_pos + step) % buf.len(); 
        //if current_pos == buf.len() - 1 {
        buf.push((i + 1) as u32)
        // } else {
        //     buf.insert(current_pos + 1, (i + 1) as u32);
        // }
        //current_pos += 1;
    }
    return buf[0]
}

fn parse(inp: &str) -> u32 {
    inp.parse().unwrap()
}
