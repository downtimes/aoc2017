use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::collections::HashSet;


#[derive(Debug)]
struct Prog {
    name: String,
    weight: u32,
    child_names: Vec<String>,
}

#[derive(Debug)]
struct SumWeight {
    name: String,
    child_weight: u32,
    weight: u32,
}

impl SumWeight {
    fn get_total(&self) -> u32 {
        self.child_weight + self.weight
    }
}

//TODO: change that into a recursive function that starts from the root node
//Really stupid implementation with terrible runtime
//Just walk n times the whole thing and update all the sum stuff
fn get_sum_weights(vec: &[Prog]) -> Vec<SumWeight> {
    //Initialize the result vector
    let mut sum_weights = vec![];
    for prog in vec.iter() {
        sum_weights.push(SumWeight{name: prog.name.clone(), child_weight: 0, weight: prog.weight});
    }

    //run over it n times and update 
    //(we need n and not sqrt(n) because we know nothing of the balance of the tree)
    for _ in 0..vec.len() {
        for (i, prog) in vec.iter().enumerate() {
            let mut sum = 0;
            for child in prog.child_names.iter() {
                let child_pos = vec.iter().position(|prog| prog.name == *child).unwrap();
                sum += sum_weights[child_pos].get_total();
            }
            sum_weights[i].child_weight = sum;
        }
    }
    sum_weights
}

//This program is absolut bonkers!
//Run through all the programs once and check the conditions for each of their children
//by taking the sums at the same index from the second parameter as help
fn find_correct_weight(progs: &[Prog], sums:&[SumWeight]) -> (String, u32) {
    for prog in progs.iter() {
        let mut child_values = vec![];
        for child_name in prog.child_names.iter() {
            child_values.push(sums.iter().find(|item| item.name == *child_name).unwrap());
        }
        if !child_values.is_empty() {
            let pivot = child_values[0].get_total();
            let unequal_pivot: Vec<_> = child_values.iter().filter(|sum| sum.get_total() != pivot).collect();
            if unequal_pivot.len() == 0 {
                continue;
            }
            if unequal_pivot.len() == 1 {
                println!("We found another item than pivot");
                let new_weight = unequal_pivot[0].weight - (unequal_pivot[0].get_total() - pivot);
                return (unequal_pivot[0].name.clone(), new_weight)
            }
            if unequal_pivot.len() == child_values.len() - 1 {
                println!("The pivot was the item");
                let new_weight = child_values[0].weight - (pivot - child_values[1].get_total());
                return (child_values[0].name.clone(), new_weight)
            }
        }
    }
    ("".to_owned(), 0)
}

fn parse_input(path: &Path) -> Vec<Prog> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let towerdefs: Vec<&str> = contents.split("\n").filter(|line| line != &"").collect();
    let cleandefs = towerdefs.iter().map(|line| {
        line.split_whitespace()
            .filter(|item| item != &"->")
            .map(|item| item.trim_matches(')').trim_matches('(').trim_matches(','))
            .collect::<Vec<_>>()
    });

    let mut res = Vec::new();
    for def in cleandefs {
        let name = def[0];
        let weight = def[1];
        let mut children = vec![];
        for &child_name in def.iter().skip(2) {
            children.push(child_name.to_owned());
        }
        res.push(Prog {
            name: name.to_owned(),
            weight: weight.parse::<u32>().unwrap(),
            child_names: children,
        });
    }
    res
}

//Root is the one node that is not a child of any other node
//so we just put every node that is listed as a child in a set
//and query the set with all names and the one who is not in the
//set is logically the root node
fn find_root(vec: &[Prog]) -> String {
    let mut is_child = HashSet::new();
    for tower in vec.iter() {
        for child_name in tower.child_names.iter() {
            is_child.insert(child_name);
        }
    }
    for tower in vec.iter() {
        if !is_child.contains(&tower.name) {
            return tower.name.clone()
        }
    }
    "".to_owned()
}

//First program that needs an optimized build to run in acceptable time
fn main() {
    let input = parse_input(Path::new("./input"));
    println!("The root of the tower is {}", find_root(&input));
    let sum_weights = get_sum_weights(&input);
    println!("we have: {:?} ", find_correct_weight(&input, &sum_weights));
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_case() {
        let vec = parse_input(Path::new("./testinput"));
        let name_root = find_root(&vec);
        assert_eq!("tknk", name_root);
    }

    #[test]
    fn test_sum() {
        let vec = parse_input(Path::new("./testinput"));
        let sum_vec = get_sum_weights(&vec);
        println!("{:?}", sum_vec);
    }
}
