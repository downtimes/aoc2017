
use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("./input").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let input = input
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    //Task 1
    let count = count_pairs(&input, 1);
    println!(
        "The result of all the numbers in Task1 you search is {}",
        count
    );

    //Task 2
    let count = count_pairs(&input, input.len() / 2);
    println!(
        "The result of all the numbers in Task2 you search is {}",
        count
    );
}

fn count_pairs(input: &[u32], step: usize) -> u32 {
    let pairs_iter = input.iter().zip(input.iter().cycle().skip(step));
    let count: u32 = pairs_iter.filter(|&(x, y)| x == y).map(|(x, _)| x).sum();

    count
}


#[cfg(test)]
pub mod test {
    use super::*;

    static TEST1: &'static str = "1122";
    static TEST2: &'static str = "1111";
    static TEST3: &'static str = "1234";
    static TEST4: &'static str = "91212129";

    const TEST1_RES: u32 = 3;
    const TEST2_RES: u32 = 4;
    const TEST3_RES: u32 = 0;
    const TEST4_RES: u32 = 9;

    #[test]
    pub fn test1() {
        let input = TEST1
            .trim()
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        let count = count_pairs(&input, 1);
        assert_eq!(TEST1_RES, count);
    }
    #[test]
    pub fn test2() {
        let input = TEST2
            .trim()
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        let count = count_pairs(&input, 1);
        assert_eq!(TEST2_RES, count);
    }
    #[test]
    pub fn test3() {
        let input = TEST3
            .trim()
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        let count = count_pairs(&input, 1);
        assert_eq!(TEST3_RES, count);
    }
    #[test]
    pub fn test4() {
        let input = TEST4
            .trim()
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        let count = count_pairs(&input, 1);
        assert_eq!(TEST4_RES, count);
    }
}
