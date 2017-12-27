fn main() {
    let input = parse(include_str!("../input"));
    println!("The shortest way to child is {} steps away",
            calculate_distance(&input));
    println!("The maximum distance away from the center was: {}", 
            calculate_max_distance(&input));
}

fn parse(input: &str) -> Vec<Direction> {
    use Direction::*;
    input.trim().split(",").map(|dir| {
        match dir {
            "ne" => NE,
            "nw" => NW,
            "n" => N,
            "se" => SE,
            "sw" => SW,
            "s" => S,
            _ => panic!("malformed input can't convert {}", dir),
        }
    }).collect()
}

#[derive(Copy, Clone)]
enum Direction {
    NW,
    N,
    NE,
    SE,
    S,
    SW,
}

//Part2
fn calculate_max_distance(child_trace: &[Direction]) -> usize {
    let mut pos = (0, 0, 0);
    let mut max_dist = 0;
    for amove in child_trace {
        pos = move_in_hexgrid(pos, *amove);
        max_dist = std::cmp::max(max_dist, distance_to_center(pos));
    }
    max_dist
}

//Part1
fn calculate_distance(child_trace: &[Direction]) -> usize {
    //Start at the origin of the hexgrid
    let mut pos = (0, 0, 0);
    for amove in child_trace {
        pos = move_in_hexgrid(pos, *amove);
    }
    distance_to_center(pos)
}

fn move_in_hexgrid(coord: (isize, isize, isize), dir: Direction) -> (isize, isize, isize) {
    //Move in our three axis system along the direction specified
    use Direction::*;
    match dir {
        NW => (coord.0 - 1, coord.1, coord.2 -1),
        NE => (coord.0 + 1, coord.1 + 1, coord.2),
        N => (coord.0, coord.1 + 1, coord.2 - 1),
        S => (coord.0, coord.1 - 1, coord.2 + 1),
        SE => (coord.0 + 1, coord.1, coord.2 +1),
        SW => (coord.0 - 1, coord.1 - 1, coord.2),
    }
}

fn distance_to_center(hex_idx: (isize, isize, isize)) -> usize {
    //The distance to the senter is just the maximum of all the axis
    let (x, y, z) = hex_idx;
    std::cmp::max(x.abs(),std::cmp::max(y.abs(), z.abs())) as usize
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_distance() {
        let input = ["ne,ne,ne", "ne,ne,sw,sw", "ne,ne,s,s", "se,sw,se,sw,sw"];
        let expected = [3, 0, 2, 3];
        for (inp, exp) in input.iter().zip(expected.iter()) {
            let parsed = parse(&inp);
            assert_eq!(*exp, calculate_distance(&parsed));
        }
    }

    #[test]
    fn test_distance_from_orig() {
        assert_eq!(0, distance_to_center((0, 0, 0)));
        assert_eq!(1, distance_to_center((1, 1, 0)));
        assert_eq!(2, distance_to_center((2, 2, 0)));
        assert_eq!(3, distance_to_center((1, 3, -2)));
        assert_eq!(1, distance_to_center((-1, 0, -1)));
        assert_eq!(2, distance_to_center((-1, 1, -2)));
        assert_eq!(3, distance_to_center((-1, -3, 2)));
    }

    #[test]
    fn test_move() {
        use Direction::*;
        assert_eq!((0, -1, 1), move_in_hexgrid((0, 0, 0), S));
        assert_eq!((1, 0, 1), move_in_hexgrid((0, 0, 0), SE));
        assert_eq!((1, 1, 0), move_in_hexgrid((0, 0, 0), NE));
        assert_eq!((0, 1, -1), move_in_hexgrid((0, 0, 0), N));
        assert_eq!((-1, 0, -1), move_in_hexgrid((0, 0, 0), NW));
        assert_eq!((-1, -1, 0), move_in_hexgrid((0, 0, 0), SW));
        assert_eq!(move_in_hexgrid((1, 0, 1), NE), move_in_hexgrid((1, 1, 0), SE));
    }
}