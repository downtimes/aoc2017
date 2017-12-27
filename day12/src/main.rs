use std::collections::HashSet;

#[allow(dead_code)]
const TEST_INPUT: &str = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5\n";

//NOTE: A little overenginered. Was the undirected graph really necessary?
fn main() {
    let input = parse(include_str!("../input"));
    let graph = build_graph(input);
    println!("The number of programs in a group with program 0 is: {:?}", 
            graph.get_group(0).len());
    println!("The number of independent groups is: {}", 
            graph.count_groups());
}

#[derive(Debug)]
struct Graph {
    index: Vec<usize>,
    adjacent: Vec<usize>,
}

impl Graph {
    fn count_groups(&self) -> usize {
        let mut res = 1;
        let mut all_cliques = self.get_group(0);

        for node_id in 0..(self.index.len() - 1) {
            if !all_cliques.contains(&node_id) {
                res += 1;
                all_cliques.extend(self.get_group(node_id));
            }
        }
        res
    }

    fn get_adj_nodes(&self, node_id: usize) -> &[usize] {
        if node_id == self.index.len() - 1 {
            &self.adjacent[self.index[node_id]..]
        } else {
            &self.adjacent[self.index[node_id]..self.index[node_id+1]]
        }
    }

    //I think I have accidentally reinvented DFS here?
    fn get_group(&self, node_id: usize) -> HashSet<usize> {
        let mut res = HashSet::new();
        res.insert(node_id);
        let mut to_inspect = vec![];
        to_inspect.extend(self.get_adj_nodes(node_id));
        while !to_inspect.is_empty() {
            //we checked for empty so this unwrap is always safe;
            let new_node = to_inspect.pop().unwrap();
            if !res.contains(&new_node) {
                //Put the new nodes we want to inspect in the set
                to_inspect.extend(self.get_adj_nodes(new_node));
                res.insert(new_node);
            }
        }
        res
    }
}

fn build_graph(inp: Vec<Vec<usize>>) -> Graph {
    //Get our indixes into our final adjlist from the calculated adjlists
    let mut sum = 0;
    let mut index = vec![0; inp.len()];
    for (idx, list) in inp.iter().enumerate() {
        index[idx as usize] = sum;
        sum += list.len();
    }

    Graph{
        index: index,
        adjacent: inp.into_iter().flat_map(|vec| vec.into_iter()).collect(),
    }
}


//TODO: The parsing is pretty.... bad...
fn parse(inp: &str) -> Vec<Vec<usize>> {
    inp.trim().split("\n").map(|line| {
        let (_, child_list) = line.split_at(
            line.find(">").expect("There was no pipe in the input") + 1);
        child_list.split(',').map(|id| {
            id.trim().parse::<usize>()
            .expect("Malformed input, expected a u32 on the right side of <->")
        }).collect()
    }).collect()
}
