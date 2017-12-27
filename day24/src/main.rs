use std::io::{self, Read};

type Bridge = Vec<Pipe>;

#[derive(Debug, Clone)]
struct Pipe {
    value: u32,
    unused_ends: Vec<u32>,
}

impl Pipe {
    fn has_end(&self, val: u32) -> bool {
        self.unused_ends.contains(&val)
    }

    fn remove_end(&mut self, val: u32) {
        let first_idx = self.unused_ends.iter().position(|&x| x == val).expect("No such end!");
        self.unused_ends.swap_remove(first_idx);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let pipes = parse(&input);
    let bridges = create_all_bridges(pipes);
    println!("the strongest bridge: {:?}", bridges.iter().map(|bridge| { 
        bridge.iter().map(|pipe| {
            pipe.value
        }).sum::<u32>() 
    }).max().unwrap());

    //Part 2
    let mut long_strong = vec![];
    let mut strengh = 0;
    for bridge in bridges {
        if long_strong.len() < bridge.len() {
            long_strong = bridge;
            strengh = long_strong.iter().map(|pipe| pipe.value).sum();
        } else if long_strong.len() == bridge.len() {
            let strengh_new = bridge.iter().map(|pipe| pipe.value).sum();
            if strengh < strengh_new {
                strengh = strengh_new;
                long_strong = bridge;
            }
        }
    }
    println!("the longest bridge that is strong: {}\nand {} long.", strengh, long_strong.len());
}


fn create_all_bridges(pipes: Vec<Pipe>) -> Vec<Bridge> {
    let mut res: Vec<Vec<Pipe>> = vec![];
    for (idx, start) in pipes.iter().enumerate().filter(|&(_, pipe)| pipe.has_end(0)) {
        let mut start = start.clone();
        start.remove_end(0);
        let path = vec![];
        let mut rest = pipes.clone();
        rest.remove(idx);
        let mut sol = create(start.clone(), rest.clone(), path);
        res.append(&mut sol);
    }
    res
}

fn create(last: Pipe, to_consider: Vec<Pipe>, path: Bridge) -> Vec<Bridge> {
    //this works because we removed the other end before the invocation
    //so only one end is left
    let number_used = last.unused_ends[0];
    let mut path = path;
    path.push(last);
    //we have no more ends that could connect. Our path is complete
    if to_consider.iter().all(|pipe| !pipe.has_end(number_used)) {
        return vec![path];
    }
    let mut res = vec![];
    for (idx, consider) in to_consider.iter().enumerate().filter(|&(_, pipe)| pipe.has_end(number_used)) {
       //make a new list to consider with our pipe removed
       let mut next_consider = to_consider.clone();
       next_consider.swap_remove(idx);
       //remove the pipe end we used
       let mut consider = consider.clone();
       consider.remove_end(number_used);
       //all results from lower invokations get accumulated here
       res.append(&mut create(consider, next_consider, path.clone()));
    }
    return res;
}

fn parse(inp: &str) -> Vec<Pipe> {
    let lines = inp.trim().lines();
    let mut res = vec![];
    for line in lines {
        let parts = line.split('/');
        let unused = parts.map(|num| num.parse().unwrap()).collect::<Vec<_>>();
        let pipe = Pipe {
            value: unused.iter().sum(),
            unused_ends: unused,
        };
        res.push(pipe);
    }
    res
}