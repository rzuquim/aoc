use std::collections::HashMap;

use utils::io::yield_lines_trimmed;

fn main() {
    let (input_file, verbose) = utils::io::parse_args();
    let part_one = solve_part_1(&input_file, verbose);
    println!("Part one: {:?}", part_one);
    let part_two = solve_part_2(&input_file, verbose);
    println!("Part two: {:?}", part_two);
}

pub fn solve_part_1(input_file: &str, verbose: bool) -> i32 {
    let mut diff_sum = 0;
    let mut evaluated_idx = 0;
    let (mut list_one, mut list_two) = load_lists(input_file, verbose);

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

fn solve_part_2(input_file: &str, verbose: bool) -> i32 {
    let (list_one, list_two) = load_lists(input_file, verbose);
    let mut count_per_number = HashMap::<i32, i32>::new();

    for n in list_two {
        let entry = count_per_number.entry(n);
        if verbose {
            println!("{entry:?} @ list 2");
        }
        *entry.or_insert(0) += 1;
    }

    let mut similarity_score = 0;
    for n in list_one {
        let list_two_occurrences = count_per_number.get(&n).unwrap_or(&0);
        similarity_score += n * list_two_occurrences;

        if verbose {
            println!("similarity_score = {n} * {list_two_occurrences}");
        }
    }
    return similarity_score;
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

fn load_lists(input_file: &str, verbose: bool) -> (Vec<i32>, Vec<i32>) {
    let mut list_one = Vec::<i32>::new();
    let mut list_two = Vec::<i32>::new();

    for line in yield_lines_trimmed(input_file) {
        let (item_one, item_two) = line.split_once(' ').unwrap();
        list_one.push(
            item_one
                .trim()
                .parse::<i32>()
                .expect("data must be a number"),
        );
        list_two.push(
            item_two
                .trim()
                .parse::<i32>()
                .expect("data must be a number"),
        );
    }

    if verbose {
        println!("List one: {:?}", list_one);
        println!("List two: {:?}", list_two);
    }

    return (list_one, list_two);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part_one() {
        let part_one = solve_part_1("./data/sample.txt", false);
        assert_eq!(part_one, 11);
    }

    #[test]
    fn test_part_two() {
        let part_two = solve_part_2("./data/sample.txt", true);
        assert_eq!(part_two, 31);
    }
}
