use std::fs;

pub fn run() {
    let input = fs::read_to_string("data/day_4.txt").expect("unable to read file");
    println!("{}", first(&input));
    println!("{}", second(&input));
}

struct Section {
    start: u32,
    end: u32,
}

fn parse_assignments(input: &str) -> (Section, Section) {
    let input = input
        .split_once(',')
        .expect("expected a pair of assignments");
    let a = input.0.split_once('-').expect("range expected");
    let b = input.1.split_once('-').expect("range expected");

    (
        Section {
            start: a.0.parse::<u32>().expect("number expected"),
            end: a.1.parse::<u32>().expect("number expected"),
        },
        Section {
            start: b.0.parse::<u32>().expect("number expected"),
            end: b.1.parse::<u32>().expect("number expected"),
        },
    )
}

fn first(input: &str) -> u32 {
    u32::try_from(
        input
            .lines()
            .map(parse_assignments)
            .filter(|(a1, a2)| {
                a1.start >= a2.start && a1.end <= a2.end || a2.start >= a1.start && a2.end <= a1.end
            })
            .count(),
    )
    .expect("unexpected overflow")
}

fn second(input: &str) -> u32 {
    u32::try_from(
        input
            .lines()
            .map(parse_assignments)
            .filter(|(a1, a2)| {
                a1.start >= a2.start && a1.start <= a2.end
                    || a2.start >= a1.start && a2.start <= a1.end
            })
            .count(),
    )
    .expect("unexpected overflow")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first() {
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
        assert_eq!(2, first(&input));
    }

    #[test]
    fn test_second() {
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
        assert_eq!(4, second(&input));
    }
}
