use std::fs;

pub fn run() {
    let input = fs::read_to_string("data/day_8.txt").expect("unable to read file");
    let grid = parse_grid(&input);
    println!("{}", first(&grid));
    println!("{}", second(&grid));
}

fn parse_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|r| {
            r.chars()
                .map(|h| h.to_digit(10).expect("expected a number"))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

fn first(grid: &Vec<Vec<u32>>) -> u32 {
    let edges = u32::try_from(grid.len() * 2 + grid[0].len() * 2 - 4).expect("expected a u32");

    u32::try_from(
        (1..grid.len() - 1)
            .map(|y| {
                (1..grid[y].len() - 1)
                    .map(|x| is_visible(grid, x, y))
                    .filter(|v| *v)
                    .count()
            })
            .sum::<usize>(),
    )
    .expect("expected a u32")
        + edges
}

fn is_visible(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    (0..x)
        .rev()
        .map_while(|v| {
            if grid[y][x] > grid[y][v] {
                Some(true)
            } else {
                None
            }
        })
        .count()
        == x
        || (x + 1..grid[y].len())
            .map_while(|v| {
                if grid[y][x] > grid[y][v] {
                    Some(true)
                } else {
                    None
                }
            })
            .count()
            == grid[y].len() - x - 1
        || (0..y)
            .rev()
            .map_while(|v| {
                if grid[y][x] > grid[v][x] {
                    Some(true)
                } else {
                    None
                }
            })
            .count()
            == y
        || (y + 1..grid.len())
            .map_while(|v| {
                if grid[y][x] > grid[v][x] {
                    Some(true)
                } else {
                    None
                }
            })
            .count()
            == grid.len() - 1 - y
}

fn second(grid: &Vec<Vec<u32>>) -> u32 {
    (1..grid.len() - 1)
        .map(|y| {
            (1..grid[y].len() - 1)
                .map(|x| scenic_score(grid, x, y))
                .max()
                .expect("expected a value")
        })
        .max()
        .expect("expected a number")
}

fn scenic_score(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let mut top = 0;
    for v in (0..y).rev() {
        top += 1;
        if grid[y][x] <= grid[v][x] {
            break;
        }
    }

    let mut bottom = 0;
    for v in y + 1..grid.len() {
        bottom += 1;
        if grid[y][x] <= grid[v][x] {
            break;
        }
    }

    let mut left = 0;
    for v in (0..x).rev() {
        left += 1;
        if grid[y][x] <= grid[y][v] {
            break;
        }
    }

    let mut right = 0;
    for v in x + 1..grid[y].len() {
        right += 1;
        if grid[y][x] <= grid[y][v] {
            break;
        }
    }

    top * bottom * left * right
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first() {
        let input = "30373
25512
65332
33549
35390";
        assert_eq!(21, first(&parse_grid(&input)));
    }

    #[test]
    fn test_second() {
        let input = "30373
25512
65332
33549
35390";
        assert_eq!(8, second(&parse_grid(&input)));
    }
}
