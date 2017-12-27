use std::fs::File;
use std::io::Read;

fn get_input() -> Vec<i32> {
    let mut file = File::open("./input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.split("\n").filter(|line| line != &"").map(|line| line.parse::<i32>().unwrap()).collect()
}

// Nicht schön aber funktioniert. Da hätte man mit sicherheit was mehr im 
// functional style machen können das die abfrage einfach composition basiert
// löst und damit den gleichen code wiederverwenden können
fn main() {
    //Part1
    let mut memory = get_input();
    let mut head_idx: i32 = 0;
    let mut step= 0;
    while (head_idx as usize) < memory.len() {
        let value = &mut memory[head_idx as usize];
        head_idx += *value;
        *value += 1;
        step += 1;
    }

    println!("It took {} steps to break out of the maze", step);

    //Part2
    let mut memory = get_input();
    let mut head_idx: i32 = 0;
    let mut step= 0;
    while (head_idx as usize) < memory.len() {
        let value = &mut memory[head_idx as usize];
        head_idx += *value;
        *value = if *value >= 3 { *value - 1 } else { *value + 1 };
        step += 1;
    }

    println!("It took {} steps to break out of the maze", step);
}

#[allow(unused)]
fn get_test_input() -> Vec<i32> {
    vec![0,3,0,1,-3]
}
