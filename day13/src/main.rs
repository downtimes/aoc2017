fn main() {
    //Part1
    let firewall = parse(include_str!("../input"));
    let sum = calculate_severity(firewall);
    println!("The severity is: {}", sum);
    
    //Part2 implementation is not crazy good. With optimizations this takes up
    //to 10 seconds...
    let mut firewall = parse(include_str!("../input"));
    let mut delay = 0;
    for i in 0.. {
        if !has_collision(firewall.clone()) {
            delay = i;
            break;
        }
        step_picosecond(&mut firewall);
    }
    println!("The searched delay is: {}", delay);
}

fn has_collision(firewall: Vec<Option<FirewallState>>) -> bool {
    let mut firewall = firewall;

    let mut collided = false;
    let hops = firewall.len();
    for position in 0..hops {
        if let Some(ref wall) = firewall[position] {
            //We collided with the firewall for the move
            if wall.position == 0 {
                collided = true;
                break;
            }
        }
        step_picosecond(&mut firewall);
    }

    collided
}


fn calculate_severity(firewall: Vec<Option<FirewallState>>) -> usize {
    let mut firewall = firewall;
    //Run our packet through the firewall
    let mut sum = 0;
    let hops = firewall.len();
    for position in 0..hops {
        if let Some(ref wall) = firewall[position] {
            //We collided with the firewall for the move
            if wall.position == 0 {
                sum += position * wall.range as usize;
            }
        }
        step_picosecond(&mut firewall);
    }
    return sum;
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    UP,
    DOWN,
    NONE,
}

#[derive(Debug, Copy, Clone)]
struct FirewallState {
    range: u32,
    position: u32,
    dir: Direction,
}

fn step_picosecond(firewall: &mut [Option<FirewallState>]) {
    for slot in firewall.iter_mut() {
        if let &mut Some(ref mut wall) = slot {
            match wall.dir {
                Direction::UP => {
                    wall.position -= 1;
                    if wall.position == 0 {
                        wall.dir = Direction::DOWN;
                    }
                }
                Direction::DOWN => {
                    wall.position += 1;
                    if wall.position == wall.range - 1 {
                        wall.dir = Direction::UP;
                    }
                }
                Direction::NONE => {}
            }

        }
    }
}

//Slightly bad modelling because of the sparseness we have many empty fields
//doesn't matter with a not so deep firewall though
fn parse(inp: &str) -> Vec<Option<FirewallState>> {
    let mut input_iter = inp.trim().split("\n").map(|line| {
        let mut items = line.split(":");
        let depth = items.next().unwrap().trim().parse::<u32>().unwrap();
        (depth, items.next().unwrap().trim().parse::<u32>().unwrap())
    });
    let mut res = Vec::new();
    let mut current_item = input_iter.next();
    for i in 0.. {
        match current_item {
            None => break,
            Some(item) if item.0 == i => {
                res.push(Some(FirewallState {
                    range: item.1,
                    position: 0,
                    dir: if item.1 != 1 { Direction::DOWN } else { Direction::NONE },
                }));
                current_item = input_iter.next();
            }
            Some(_) => res.push(None),
        }
    }
    res
}
