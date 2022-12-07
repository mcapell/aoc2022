use std::{collections::HashSet, fs};

pub fn run() {
    let input = fs::read_to_string("data/day_6.txt").expect("unable to read file");
    println!("{}", find_marker(&input, 4));
    println!("{}", find_marker(&input, 14));
}

fn find_marker(input: &str, size: usize) -> usize {
    let input = input.chars().collect::<Vec<char>>();
    (0..input.len() - size)
        .map(|i| input.get(i..i + size).expect("value expected"))
        .map_while(all_unique)
        .count()
        + size
}

fn all_unique(value: &[char]) -> Option<bool> {
    if value.len() == value.iter().collect::<HashSet<&char>>().len() {
        None
    } else {
        Some(true)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(7, find_marker(&input, 4));
    }

    #[test]
    fn test_second() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(19, find_marker(&input, 14));
    }
}
