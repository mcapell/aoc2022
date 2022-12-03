use std::{collections::HashSet, fs};

pub fn run() {
    let input = fs::read_to_string("data/day_3.txt").expect("unable to read file");
    println!("{}", first(&input));
    println!("{}", second(&input));
}

fn find_duplicate_item(comp1: &str, comp2: &str) -> char {
    comp1
        .chars()
        .filter_map(|c| comp2.chars().find(|e| c == *e))
        .into_iter()
        .next()
        .expect("expected a duplicate item")
}

fn priority(c: char) -> u32 {
    let mut a = ('a'..='z').collect::<Vec<_>>();
    let b = ('A'..='Z').collect::<Vec<_>>();
    a.extend(b);

    u32::try_from(
        1 + a
            .iter()
            .position(|e| *e == c)
            .expect("expected the value to be in a-zA-z"),
    )
    .expect("this should never overflow")
}

fn first(input: &str) -> u32 {
    input
        .lines()
        .map(|s| s.split_at(s.len() / 2))
        .map(|(a, b)| priority(find_duplicate_item(a, b)))
        .sum()
}

fn second(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();
    (0..lines.len() / 3)
        .map(|i| {
            let i = i * 3;
            let g1 = lines[i].chars().collect::<HashSet<char>>();
            let g2 = lines[i + 1].chars().collect::<HashSet<char>>();
            let g3 = lines[i + 2].chars().collect::<HashSet<char>>();

            priority(
                **g1.intersection(&g2)
                    .copied()
                    .collect::<HashSet<char>>()
                    .intersection(&g3)
                    .collect::<Vec<&char>>()
                    .first()
                    .expect("expected a common element for group"),
            )
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZTJ\nCrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(157, first(&input));
    }

    #[test]
    fn test_second() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZTJ\nCrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(70, second(&input));
    }

    #[test]
    fn find_duplicate() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let input = input.split_at(input.len() / 2);
        assert_eq!('p', find_duplicate_item(input.0, input.1));
    }

    #[test]
    fn test_priority() {
        assert_eq!(1, priority('a'));
        assert_eq!(27, priority('A'));
    }
}
