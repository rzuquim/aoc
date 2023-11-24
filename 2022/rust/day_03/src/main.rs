use std::collections::{HashSet};

mod utils;

fn main() {
    let (input_file, verbose) = utils::parse_args();
    let (part_one, part_two) = solve(&input_file, verbose);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}

fn solve(input_file: &str, verbose: bool) -> (u32, i32) {
    let mut total_priority = 0;
    for line in utils::yield_lines_trimmed(input_file) {
        let rucksack = parse_rucksack(&line, verbose);
        let common_items = common_items_in_compartments(&rucksack, verbose);

        let line_priority = calculate_priority_by_commonality(common_items);
        if verbose {
            println!("Line priority {}", line_priority);
        }

        total_priority += line_priority;
    }
    return (total_priority, -1);
}

fn parse_rucksack(line: &String, verbose: bool) -> Rucksack {
    let mid = line.len() / 2;
    let (compartment_a, compartment_b) = (&line[0..mid], &line[mid..]);

    if verbose {
        println!("Compartments {}, {}", compartment_a, compartment_b);
    }

    return Rucksack {
        compartment_a: compartment_a.chars().collect::<HashSet<char>>(),
        compartment_b: compartment_b.chars().collect::<HashSet<char>>(),
    };
}

fn common_items_in_compartments<'a>(
    rucksack: &'a Rucksack,
    verbose: bool,
) -> impl Iterator<Item = &'a char> {
    let common_items = rucksack.compartment_a.intersection(&rucksack.compartment_b);
    if verbose {
        println!("Common items in compartments {:?}", common_items);
    }
    return common_items;
}

fn calculate_priority_by_commonality<'a>(common_items: impl Iterator<Item = &'a char>) -> u32 {
    common_items
        .map(|letter| {
            if letter.is_lowercase() {
                (*letter as u32) - ('a' as u32) + 1
            } else {
                (*letter as u32) - ('A' as u32) + 27
            }
        })
        .sum()
}

struct Rucksack {
    compartment_a: HashSet<char>,
    compartment_b: HashSet<char>,
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_part_one() {
        let (part_one_solved, _) = solve("./data_input.txt", false);
        assert_eq!(part_one_solved, 7967);
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let (_, part_two_solved) = solve("./data_input.txt", false);
        assert_eq!(part_two_solved, -1);
    }
}
