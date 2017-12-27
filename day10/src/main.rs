use std::iter::FromIterator;

fn parse_input2(inp: &str) -> Vec<u8> {
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

fn parse_input1(inp: &str) -> Vec<u8> {
    inp.trim()
        .split(",")
        .map(|elem| {
            elem.trim().parse::<u8>().expect(
                "The input was not formed correctly and had values that are not in the range 0..255",
            )
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

#[allow(dead_code)]
const TEST_INPUT: &'static str = "3,4,1,5";

const ROPE_LENGTH: u8 = 255;
const ROUNDS: usize = 64;
const APPEND_LENGHTS: [u8; 5] = [17, 31, 73, 47, 23];

fn main() {
    //Part1
    let input = parse_input1(include_str!("../input"));
    let mut rope = Vec::from_iter((0..ROPE_LENGTH));
    //stupid but range doesn't accept 256 because out of range of u8!
    rope.push(ROPE_LENGTH);
    let mut position = 0;
    let mut skip = 0;
    for inp in input {
        reverse_subrope(position, position + inp as usize, &mut rope[..]);
        position = (position + inp as usize + skip) % rope.len();
        skip += 1;
    }
    println!(
        "The requested number is: {}",
        rope[0] as u16 * rope[1] as u16
    );

    //Part2
    let mut input = parse_input2(include_str!("../input"));
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
    let hex_string = dense_hash
        .iter()
        .map(|elem| format!("{:02x}", *elem))
        .collect::<String>();
    println!("Knot hash is: {}", hex_string);
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
