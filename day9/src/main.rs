fn calculate(input_chars: &Vec<char>) -> (usize, usize) {
    let mut index: usize = 0;
    let mut nesting_level = 0;
    let mut sum = 0;
    let mut garbage = 0;
    //Iterate over all the input and if we find garbage discard it and move
    //the index on for the discarded ammount. Also keep track of which nesting 
    //level we entered with each opening "{" and closing "}"
    while let Some(c) = input_chars.get(index) {
        index += 1;
        match *c {
            '<' => {
                let (new_index, deleted) = delete_garbage(index, &input_chars);
                garbage += deleted;
                index = new_index;
            }
            '{' => {
                nesting_level += 1;
                sum += nesting_level;
            }
            '}' => nesting_level -= 1,
            ',' => {}
            _ => panic!("unexpected symbol found in stream"),
        }
    }
    (sum, garbage)
}

fn delete_garbage(index: usize, input: &Vec<char>) -> (usize, usize) {
    let mut idx = index;
    let mut last_char = '<';
    let mut deleted = 0;
    while let Some(c) = input.get(idx) {
        idx += 1;
        if last_char != '!' {
            if *c == '>' {
                return (idx, deleted);
            }
            if *c != '!' {
                deleted += 1;
            }
            last_char = *c;
        } else {
            if *c == '!' {
                last_char = ' ';
            } else {
                last_char = *c;
            }
        }
    }
    (idx, deleted)
}

fn main() {
    let input = include_str!("../input");
    let input_chars: Vec<char> = input.chars().collect();
    //Part1
    //Part2
    let (sum, garbage) = calculate(&input_chars);
    println!(
        "The requested sum is {}, and the garbage is {}",
        sum, garbage
    );

    //Nice solution
    let (sum, garbage) = calculate_clean(&input_chars);
    println!(
        "The requested sum is {}, and the garbage is {}",
        sum, garbage
    )
}

//Because I was unhappy with how I wrote it here is a MUCH clearer version with 
//a state machine
fn calculate_clean(input: &Vec<char>) -> (usize, usize) {
    use State::*;
    let mut chars = input.iter();
    let mut state = READ;
    let mut sum = 0;
    let mut nested = 0;
    let mut garbage = 0;
    while let Some(c) = chars.next() {
        match state {
            READ => match *c {
                '{' => {
                    nested += 1;
                    sum += nested
                }
                '}' => nested -= 1,
                '<' => state = GARBAGE,
                ',' => {}
                _ => panic!("Malformed input"),
            },
            GARBAGE => match *c {
                '!' => state = IGNORE,
                '>' => state = READ,
                _ => garbage += 1,
            },
            IGNORE => {
                state = GARBAGE;
            }
        }
    }
    (sum, garbage)
}

enum State {
    IGNORE,
    READ,
    GARBAGE,
}
