use std::collections::HashMap;

use utils::io::yield_lines_trimmed;

fn main() {
    let (input_file, verbose) = utils::io::parse_args();
    let dirs_sizes = calc_dirs_sizes(&input_file, verbose);
    let part_one = solve_part_one(&dirs_sizes, 100000);
    let part_two = solve_part_two(&dirs_sizes, 70000000, 30000000, verbose);

    println!("Part one: {:?}", part_one);
    println!("Part two: {:?}", part_two);
}

fn solve_part_one(dirs_sizes: &HashMap<String, usize>, threshold: usize) -> usize {
    dirs_sizes
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
        .sum()
}

fn solve_part_two(
    dirs_sizes: &HashMap<String, usize>,
    disk_size: usize,
    update_patch_size: usize,
    verbose: bool,
) -> usize {
    let total_used = dirs_sizes.get("/").expect("Dirs must contain root dir.");

    let unused_space = disk_size - total_used;
    let required_space = update_patch_size - unused_space;

    if verbose {
        println!(
            "total_used: {} / unused: {} / required: {}",
            total_used, unused_space, required_space
        );
    }

    return dirs_sizes
        .iter()
        .filter_map(|(_, &value)| {
            if value >= required_space {
                Some(value)
            } else {
                None
            }
        })
        .min()
        .expect("Could not find directory bellow the threshold");
}

fn calc_dirs_sizes(input_file: &str, verbose: bool) -> HashMap<String, usize> {
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
    return size_map;
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
                    pwd.push(path.clone());
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
    use crate::*;

    #[test]
    fn test_part_one() {
        let dirs_sizes = calc_dirs_sizes("./data_input.txt", false);
        let part_one = solve_part_one(&dirs_sizes, 100000);
        assert_eq!(part_one, 1844187);
    }

    #[test]
    fn test_part_two() {
        let dirs_sizes = calc_dirs_sizes("./data_input.txt", false);
        let part_two = solve_part_two(&dirs_sizes, 70000000, 30000000, false);
        assert_eq!(part_two, 4978279);
    }
}
