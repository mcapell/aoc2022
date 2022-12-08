use std::{collections::HashMap, fs};

pub fn run() {
    let input = fs::read_to_string("data/day_7.txt").expect("unable to read file");
    let fs = walk_fs(&input);
    println!("{}", first(&fs));
    println!("{}", second(&fs));
}

fn walk_fs(input: &str) -> HashMap<String, u32> {
    let mut fs = HashMap::new();
    let mut current_path = vec![];
    for line in input.lines() {
        let l = line.split(' ').collect::<Vec<&str>>();
        if l[0].starts_with('$') && l[1] == "cd" {
            match l.get(2) {
                Some(&"..") => {
                    current_path.pop();
                }
                _ => current_path.push(l[2]),
            }
            continue;
        }

        if !l[0].starts_with('$') && !l[0].starts_with("dir") {
            let size = l[0].parse::<u32>().expect("expected a number");

            for i in 0..current_path.len() {
                let path = current_path[0..current_path.len() - i].join("/").clone();
                match fs.get_mut(&path) {
                    Some(v) => *v += size,
                    None => {
                        fs.insert(path, size);
                    }
                }
            }
        }
    }

    fs
}

fn first(fs: &HashMap<String, u32>) -> u32 {
    fs.iter()
        .map(|(_, &v)| if v <= 100_000 { v } else { 0 })
        .sum()
}

fn second(fs: &HashMap<String, u32>) -> u32 {
    let space_needed =
        30_000_000 - (70_000_000 - fs.get(&"/".to_owned()).expect("root path expected"));

    fs.iter()
        .map(|(_, &v)| v)
        .filter(|&v| v >= space_needed)
        .min()
        .expect("a valid result is expected")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        assert_eq!(95437, first(&walk_fs(&input)));
    }

    #[test]
    fn test_second() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        assert_eq!(24933642, second(&walk_fs(&input)));
    }
}
