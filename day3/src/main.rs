fn main() {
    let input_number = 277678;
    let ring = get_included_ring(input_number);

    let ring_base = if ring != 0 {
        (1..).filter(|x| x % 2 != 0)
            .map(|x| x * x)
            .nth((ring - 1) as usize)
            .unwrap()
    } else {
        1
    };
    let ring_length = (1..).filter(|x| x % 2 != 0).nth(ring as usize).unwrap();
    let misalignment = get_misalingment_from_axis(ring_base, ring_length, input_number);
    println!("The searched number of moves is: {}", ring + misalignment);

    println!(
        "The second solution is: {}",
        get_second_solution(input_number)
    );
}

//Quick and dirty solution. With other numbers this easily panics
fn get_second_solution(input: u32) -> u32 {
    //arbitrary size and lets hope we don't hit out of bounds before we find the solution
    let mut grid = vec![vec![0; 1024]; 1024];
    let (mut currx, mut curry) = (grid.len() / 2, grid[0].len() / 2);
    let mut lengths = (1..).filter(|x| x % 2 != 0).skip(1);
    grid[currx][curry] = 1;
    currx += 1;
    loop {
        let length = lengths.next().unwrap();
        for _up in 1..(length - 1) {
            let fill = fill(&grid, currx, curry);
            if fill >= input {
                return fill;
            };
            grid[currx][curry] = fill;
            curry += 1;
        }
        for _left in 1..length {
            let fill = fill(&grid, currx, curry);
            if fill >= input {
                return fill;
            };
            grid[currx][curry] = fill;
            currx -= 1;
        }
        for _down in 1..length {
            let fill = fill(&grid, currx, curry);
            if fill >= input {
                return fill;
            };
            grid[currx][curry] = fill;
            curry -= 1;
        }
        for _right in 0..length {
            let fill = fill(&grid, currx, curry);
            if fill >= input {
                return fill;
            };
            grid[currx][curry] = fill;
            currx += 1;
        }
    }
}

fn fill(grid: &[Vec<u32>], x: usize, y: usize) -> u32 {
    let number = grid[x][y + 1] + grid[x][y - 1] + grid[x + 1][y] + grid[x - 1][y]
        + grid[x + 1][y + 1] + grid[x - 1][y - 1] + grid[x + 1][y - 1]
        + grid[x - 1][y + 1];
    number
}


fn get_misalingment_from_axis(ring_base: u32, ring_length: u32, num: u32) -> u32 {
    let mut first_number = ring_base;
    let distance = ring_length - 1;
    for _ in 0..4 {
        let next_number = first_number + distance;
        let axis = (first_number + next_number) / 2;

        if num >= (axis - (distance / 2)) && num <= (axis + (distance / 2)) {
            let dis = (axis as i32) - (num as i32);
            return dis.abs() as u32;
        }
        first_number = next_number;
    }
    ring_base
}

fn get_included_ring(num: u32) -> u32 {
    let lengths = (1u32..).filter(|x| x % 2 != 0);
    let ring = lengths
        .take_while(|x| x * x < num)
        .fold(0, |sum, _| sum + 1);
    ring
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_ring() {
        assert_eq!(1, get_included_ring(4));
        assert_eq!(1, get_included_ring(9));
        assert_eq!(0, get_included_ring(1));
        assert_eq!(2, get_included_ring(10));
        assert_eq!(2, get_included_ring(19));
        assert_eq!(3, get_included_ring(26));
    }

    #[test]
    fn test_misalignment() {
        assert_eq!(0, get_misalingment_from_axis(1, 1, 1));
        assert_eq!(0, get_misalingment_from_axis(1, 3, 2));
        assert_eq!(0, get_misalingment_from_axis(1, 3, 4));
        assert_eq!(0, get_misalingment_from_axis(1, 3, 6));
        assert_eq!(0, get_misalingment_from_axis(1, 3, 8));
        assert_eq!(1, get_misalingment_from_axis(1, 3, 3));
        assert_eq!(1, get_misalingment_from_axis(1, 3, 9));
        assert_eq!(0, get_misalingment_from_axis(9, 5, 11));
        assert_eq!(1, get_misalingment_from_axis(9, 5, 12));
        assert_eq!(2, get_misalingment_from_axis(9, 5, 13));
    }

}
