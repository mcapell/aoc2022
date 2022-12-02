use std::fs;

pub fn run() {
    let mut input = parse_input(&read_data("data/day_1.txt"));
    input.sort_unstable();
    input.reverse();
    println!("{}", first(&input));
    println!("{}", second(&input));
}

fn read_data(path: &str) -> String {
    fs::read_to_string(path).expect("unable to read file")
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|c| c.lines().filter_map(|l| l.parse::<u32>().ok()).sum())
        .collect::<Vec<u32>>()
}

fn first(input: &[u32]) -> u32 {
    *input.first().expect("expected non-empty list of numbers")
}

fn second(input: &[u32]) -> u32 {
    input.iter().take(3).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        let mut input = parse_input(&input);
        input.sort_unstable();
        input.reverse();
        assert_eq!(24000, first(&input));
    }

    #[test]
    fn test_second() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        let mut input = parse_input(&input);
        input.sort_unstable();
        input.reverse();
        assert_eq!(45000, second(&input));
    }
}
