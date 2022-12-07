use std::fs;

pub fn run() {
    let input = fs::read_to_string("data/day_5.txt").expect("unable to read file");
    let (s, m) = parse_input(&input);
    println!("{}", first(&s, &m));
    println!("{}", second(&s, &m));
}

fn parse_input(input: &str) -> (Stacks, Vec<Move>) {
    let mut stacks = input
        .lines()
        .filter(|l| !l.starts_with("move"))
        .map(|l| {
            l.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|c| match c[0] {
                    '[' => c[1],
                    _ => ' ',
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    // Remove last two arrays
    stacks.pop();
    stacks.pop();

    stacks = (0..stacks[0].len())
        .map(|i| {
            stacks
                .iter()
                .map(|inner| inner[i])
                .rev()
                .filter(|&c| c != ' ')
                .collect::<Vec<char>>()
        })
        .collect();

    let moves = input
        .lines()
        .filter(|l| l.starts_with("move"))
        .map(|l| {
            let m = l.split(' ').collect::<Vec<&str>>();
            Move {
                num: m[1].parse::<usize>().expect("expected a number"),
                from: m[3].parse::<usize>().expect("expected a number"),
                to: m[5].parse::<usize>().expect("expected a number"),
            }
        })
        .collect::<Vec<Move>>();

    (stacks, moves)
}

fn first(stack: &Stacks, moves: &[Move]) -> String {
    let mut stack = stack.clone();
    for m in moves {
        for _ in 0..m.num {
            let v = stack[m.from - 1].pop().expect("value expected");
            stack[m.to - 1].push(v);
        }
    }
    let mut s = String::new();
    for c in stack {
        s.push(c[c.len() - 1]);
    }
    s.to_string()
}

fn second(stack: &Stacks, moves: &[Move]) -> String {
    let mut stack = stack.clone();
    for m in moves {
        let mut v: Vec<char> = vec![];
        for _ in 0..m.num {
            v.push(stack[m.from - 1].pop().expect("value expected"));
        }
        v.reverse();
        stack[m.to - 1].extend(v);
    }
    let mut s = String::new();
    for c in stack {
        s.push(c[c.len() - 1]);
    }
    s.to_string()
}

type Stacks = Vec<Vec<char>>;

#[derive(Debug)]
struct Move {
    num: usize,
    from: usize,
    to: usize,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let (s, m) = parse_input(&input);
        let s = first(&s, &m);

        println!("{:?}", s);
        assert_eq!("CMZ", s);
    }

    #[test]
    fn test_second() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let (s, m) = parse_input(&input);
        let s = second(&s, &m);

        println!("{:?}", s);
        assert_eq!("MCD", s);
    }
}
