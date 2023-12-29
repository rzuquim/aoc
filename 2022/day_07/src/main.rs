use std::collections::HashMap;

use utils::io::yield_lines_trimmed;

fn main() {
    let (input_file, verbose) = utils::io::parse_args();
    let part_one = solve(&input_file, 100000, verbose);
    // let part_two = solve(&input_file, 14, verbose);

    println!("Part one: {:?}", part_one);
    // println!("Part two: {:?}", part_two);
}

fn solve(input_file: &str, threshold: usize, verbose: bool) -> usize {
    let mut pwd = Vec::<String>::new();
    let mut size_map = HashMap::<String, usize>::new();

    for line in yield_lines_trimmed(input_file) {
        let cmd = parse(&line);
        if verbose {
            println!("{:?}", cmd);
        }
        cmd.apply(&mut pwd, &mut size_map);
        if verbose {
            println!("pwd {:?}", pwd);
            println!("sizes {:?}", size_map);
        }
    }

    return size_map
        .iter()
        .filter_map(
            |(_, &value)| {
                if value < threshold {
                    Some(value)
                } else {
                    None
                }
            },
        )
        .sum::<usize>();
}

fn parse(line: &str) -> Cmd {
    let split = line.split(' ').collect::<Vec<&str>>();

    match (split[0], split[1]) {
        ("$", "ls") => Cmd::Ls,
        ("dir", _) => Cmd::LsDirListed,
        ("$", "cd") => Cmd::Cd {
            path: String::from(split[2]),
        },
        _ => Cmd::LsFileListed {
            size: split[0]
                .parse::<usize>()
                .expect(format!("Could not read file size from {}", split[0]).as_str()),
        },
    }
}

#[derive(Debug)]
enum Cmd {
    Cd { path: String },
    Ls,
    LsDirListed,
    LsFileListed { size: usize },
}

impl Cmd {
    fn apply(&self, pwd: &mut Vec<String>, size_map: &mut HashMap<String, usize>) {
        match &self {
            Cmd::Cd { path } => match path.as_str() {
                "/" => {
                    pwd.clear();
                }
                ".." => {
                    pwd.pop();
                }
                _ => {
                    pwd.push(path.clone());
                }
            },
            Cmd::LsFileListed { size } => {
                for dir in every_sub_dir(&pwd) {
                    let curr_dir_size = match size_map.get(&dir) {
                        Some(value) => value + size,
                        None => *size,
                    };
                    size_map.insert(dir, curr_dir_size);
                }
            }
            Cmd::Ls => { /* the actual data comes from LsFileListed */ }
            Cmd::LsDirListed => { /* the actual data comes from LsFileListed */ }
        };
    }
}

pub fn every_sub_dir(pwd: &Vec<String>) -> Vec<String> {
    let mut dirs = Vec::new();
    for i in 0..pwd.len() {
        dirs.push(pwd[0..=i].join("/"));
    }
    return dirs;
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_part_one() {
        let part_one_solved = solve("./data_input.txt", 100000, false);
        assert_eq!(part_one_solved, 1844187);
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let part_two_solved = solve("./data_input.txt", 14, false);
        assert_eq!(part_two_solved, 1538);
    }
}
