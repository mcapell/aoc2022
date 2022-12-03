use std::fs;

pub fn run() {
    let input = fs::read_to_string("data/day_2.txt").expect("unable to read file");
    println!("{}", run_strategy(&input, parse_round_1));
    println!("{}", run_strategy(&input, parse_round_2));
}

enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum GameResult {
    Win,
    Draw,
    Loss,
}

struct Round {
    you: Shape,
    result: GameResult,
}

fn run_strategy(input: &str, f: fn(&str) -> Round) -> u32 {
    input
        .lines()
        .map(f)
        .collect::<Vec<Round>>()
        .iter()
        .map(count_points)
        .sum::<u32>()
}

fn parse_round_1(values: &str) -> Round {
    let opponent = match values.chars().next() {
        Some(v) => match v {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            _ => panic!("unexpected left shape: {}", v),
        },
        None => panic!("unexpected left shape: "),
    };

    let you = match values.chars().nth(2) {
        Some(v) => match v {
            'X' => Shape::Rock,
            'Y' => Shape::Paper,
            'Z' => Shape::Scissors,
            _ => panic!("unexpected right shape: {}", v),
        },
        None => panic!("unexpected right shape: "),
    };

    let result = match (&you, &opponent) {
        (Shape::Rock, Shape::Rock)
        | (Shape::Paper, Shape::Paper)
        | (Shape::Scissors, Shape::Scissors) => GameResult::Draw,

        (Shape::Rock, Shape::Paper)
        | (Shape::Paper, Shape::Scissors)
        | (Shape::Scissors, Shape::Rock) => GameResult::Loss,

        (Shape::Rock, Shape::Scissors)
        | (Shape::Paper, Shape::Rock)
        | (Shape::Scissors, Shape::Paper) => GameResult::Win,
    };

    Round { you, result }
}

fn parse_round_2(values: &str) -> Round {
    let opponent = match values.chars().next() {
        Some(v) => match v {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            _ => panic!("unexpected left shape: {}", v),
        },
        None => panic!("unexpected left shape: "),
    };

    let result = match values.chars().nth(2) {
        Some(v) => match v {
            'X' => GameResult::Loss,
            'Y' => GameResult::Draw,
            'Z' => GameResult::Win,
            _ => panic!("unexpected right shape: {}", v),
        },
        None => panic!("unexpected right shape: "),
    };

    let you = match (&opponent, &result) {
        (Shape::Rock, GameResult::Loss)
        | (Shape::Paper, GameResult::Win)
        | (Shape::Scissors, GameResult::Draw) => Shape::Scissors,

        (Shape::Scissors, GameResult::Win)
        | (Shape::Rock, GameResult::Draw)
        | (Shape::Paper, GameResult::Loss) => Shape::Rock,

        (Shape::Paper, GameResult::Draw)
        | (Shape::Rock, GameResult::Win)
        | (Shape::Scissors, GameResult::Loss) => Shape::Paper,
    };

    Round { you, result }
}

fn count_points(round: &Round) -> u32 {
    let shape = match round.you {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    };

    let result = match round.result {
        GameResult::Win => 6,
        GameResult::Draw => 3,
        GameResult::Loss => 0,
    };

    shape + result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_strategy_1() {
        let input = "A Y\nB X\nC Z";
        assert_eq!(15, run_strategy(&input, parse_round_1));
    }

    #[test]
    fn test_strategy_2() {
        let input = "A Y\nB X\nC Z";
        assert_eq!(12, run_strategy(&input, parse_round_2));
    }
}
