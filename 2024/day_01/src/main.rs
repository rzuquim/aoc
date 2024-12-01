use std::i32;

use regex::Regex;
use utils::io::yield_lines_trimmed;

fn main() {
    let (input_file, verbose) = utils::io::parse_args();
    let part_one = solve(&input_file, verbose);
    println!("Part one: {:?}", part_one);
    // let part_two = solve(&input_file, 9, verbose);
    // println!("Part two: {:?}", part_two);
}

pub fn solve(input_file: &str, verbose: bool) -> i32 {
    let mut list_one = Vec::<i32>::new();
    let mut list_two = Vec::<i32>::new();

    let splitter = Regex::new(r"\s+").expect("Invalid regex");
    for line in yield_lines_trimmed(input_file) {
        let numbers: Vec<&str> = splitter.split(&line).into_iter().collect();
        list_one.push(numbers[0].parse::<i32>().expect("data must be a number"));
        list_two.push(numbers[1].parse::<i32>().expect("data must be a number"));
    }

    if verbose {
        println!("List one: {:?}", list_one);
        println!("List two: {:?}", list_two);
    }

    let mut diff_sum = 0;
    let mut evaluated_idx = 0;

    while list_one.len() > 0 {
        let (list_one_smallest_idx, list_one_smallest) = smallest_item_idx(&list_one);
        if verbose {
            println!(
                "{evaluated_idx} smallest on list one: {list_one_smallest} @ {list_one_smallest_idx}",
            );
        }

        let (list_two_smallest_idx, list_two_smallest) = smallest_item_idx(&list_two);
        if verbose {
            println!(
                "{evaluated_idx} smallest on list two: {list_two_smallest} @ {list_two_smallest_idx}",
            );
        }

        list_one.remove(list_one_smallest_idx);
        list_two.remove(list_two_smallest_idx);

        evaluated_idx += 1;
        diff_sum += distance(list_one_smallest, list_two_smallest);
    }

    return diff_sum;
}

fn smallest_item_idx(list: &Vec<i32>) -> (usize, i32) {
    let mut smallest = i32::MAX;
    let mut smallest_idx = 0;
    let mut i = 0;
    for v in list {
        if *v <= smallest {
            smallest = *v;
            smallest_idx = i;
        }
        i += 1;
    }
    return (smallest_idx, smallest);
}

fn distance(num: i32, other: i32) -> i32 {
    return if num > other {
        num - other
    } else {
        other - num
    };
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part_one() {
        let part_one = solve("./data/input.txt", false);
        assert_eq!(part_one, 11);
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        //let part_two = solve("./data/input.txt", 9, false);
        //assert_eq!(part_two, 2531);
        todo!()
    }
}
