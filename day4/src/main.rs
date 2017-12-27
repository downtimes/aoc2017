use std::fs::File;
use std::collections::HashSet;
use std::collections::HashMap;
use std::io::Read;

fn main() {
    let mut file = File::open("./input").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let valid_count = count_valid(&input, &is_valid_pw);
    println!("The number of valid passwords is: {}", valid_count);

    let valid_count2 = count_valid(&input, &is_valid_pw2);
    println!("The number of valid passwords is: {}", valid_count2);
}

fn count_valid<F>(input: &str, predicate: F) -> u32 
    where F: Fn(&str) -> bool {
    input
        .split("\n")
        .filter(|line| line != &"")
        .map(|line| predicate(line))
        .filter(|x| x == &true)
        .count() as u32
}


fn is_valid_pw2(pw: &str) -> bool {
    let words: Vec<_> = pw.trim().split_whitespace().collect();
    for i in 0..words.len() {
        for j in (i + 1)..words.len() {
            if is_anagram(words[i], words[j]) {
                return false
            }
        }
    }
    true
}


fn is_anagram(first: &str, second: &str) -> bool {
    if first.len() != second.len() {
        return false;
    }

    //count the number of occurances of each letter in a hashmap
    let mut first_letters = HashMap::new();
    for c in first.chars() {
        let entry = first_letters.entry(c).or_insert(0);
        *entry += 1;
    }

    //check if we have the same number of the same keys
    for c in second.chars() {
        if !first_letters.contains_key(&c) {
            return false;
        }
        let vcount;
        {
            let count = first_letters.get_mut(&c).unwrap();
            *count -= 1;
            vcount = *count;
        }
        if vcount == 0 {
            first_letters.remove(&c);
        }
    }
    true
}


fn is_valid_pw(pw: &str) -> bool {
    let mut set = HashSet::new();
    pw.trim()
        .split_whitespace()
        .map(|word| {
            if set.contains(word) {
                false
            } else {
                set.insert(word);
                true
            }
        })
        .fold(true, |res, isunique| res && isunique)
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_pw2() {
        let pw = "a ab abc abd abf abj";
        assert!(is_valid_pw2(pw));
        let pw = "abcde xyz ecdab";
        assert!(!is_valid_pw2(pw));
    }

    #[test]
    fn test_anagram() {
        let first = "abcde";
        let second = "abcde";
        assert!(is_anagram(first, second));
        let second = "abcdd";
        assert!(!is_anagram(first, second));
    }

    #[test]
    fn test_pw() {
        assert_eq!(true, is_valid_pw("aa bb cc dd ee"));
        assert_eq!(false, is_valid_pw("aa bb cc dd aa"));
        assert_eq!(true, is_valid_pw("aa bb cc dd aaa"));
    }
}
