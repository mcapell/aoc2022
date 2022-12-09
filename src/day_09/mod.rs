use std::{collections::HashSet, fs};

pub fn run() {
    let input = fs::read_to_string("data/day_9.txt").expect("unable to read file");
    println!("{}", run_with_size(&input, 2));
    println!("{}", run_with_size(&input, 10));
}

struct Rope {
    tail: Vec<(i32, i32)>,
    tail_positions: HashSet<(i32, i32)>,
}

impl Rope {
    fn new(num: usize) -> Rope {
        let mut p = HashSet::new();
        p.insert((0, 0));

        let mut tail = Vec::new();
        for _ in 0..num {
            tail.push((0, 0));
        }

        Rope {
            tail,
            tail_positions: p,
        }
    }
    fn move_head(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.tail[0].1 += 1,
            Direction::Down => self.tail[0].1 -= 1,
            Direction::Left => self.tail[0].0 -= 1,
            Direction::Right => self.tail[0].0 += 1,
        }

        for i in 1..self.tail.len() {
            self.tail[i] = Rope::get_next(self.tail[i - 1], self.tail[i]);
        }

        self.tail_positions.insert(self.tail[self.tail.len() - 1]);
    }

    fn get_next(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
        if head.0 == tail.0 && head.1 == tail.1 {
            // Same position
            tail
        } else if head.0 == tail.0 && head.1 > tail.1 {
            // Head is UP
            (head.0, head.1 - 1)
        } else if head.0 == tail.0 && head.1 < tail.1 {
            // Head is DOWN
            (head.0, head.1 + 1)
        } else if head.0 < tail.0 && head.1 == tail.1 {
            // Head is LEFT
            (head.0 + 1, head.1)
        } else if head.0 > tail.0 && head.1 == tail.1 {
            // Head is RIGHT
            (head.0 - 1, head.1)
        } else if (head.0 - tail.0).abs() == 1 && (head.1 - tail.1).abs() == 1 {
            // Diagonal but still touching
            tail
        } else if head.0 < tail.0 && head.1 > tail.1 {
            // 10 am
            (tail.0 - 1, tail.1 + 1)
        } else if head.0 > tail.0 && head.1 > tail.1 {
            // 2 am
            (tail.0 + 1, tail.1 + 1)
        } else if head.0 > tail.0 && head.1 < tail.1 {
            // 5 am
            (tail.0 + 1, tail.1 - 1)
        } else if head.0 < tail.0 && head.1 < tail.1 {
            // 7 am
            (tail.0 - 1, tail.1 - 1)
        } else {
            (0, 0)
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn run_with_size(input: &str, size: usize) -> u32 {
    let mut rope = Rope::new(size);

    for l in input.lines() {
        let p = l.split(' ').collect::<Vec<&str>>();
        let c = p[1].parse::<u32>().expect("expected a number");
        let d = match p.first() {
            Some(&"U") => Direction::Up,
            Some(&"D") => Direction::Down,
            Some(&"L") => Direction::Left,
            Some(&"R") => Direction::Right,
            _ => unreachable!(),
        };

        for _ in 0..c {
            rope.move_head(d);
        }
    }

    u32::try_from(rope.tail_positions.len()).expect("expected a u32")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
        assert_eq!(13, run_with_size(&input, 2));
    }

    #[test]
    fn test_second_short() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
        assert_eq!(1, run_with_size(&input, 10));
    }

    #[test]
    fn test_second_large() {
        let input = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";
        assert_eq!(36, run_with_size(&input, 10));
    }
}
