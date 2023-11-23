use std::collections::HashSet;

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
        let (compartment_a, compartment_b) = split_into_compartments(&line);
        if verbose {
            println!("Compartments {}, {}", compartment_a, compartment_b);
        }

        let unique_items_a = compartment_a.chars().collect::<HashSet<char>>();
        let unique_items_b = compartment_b.chars().collect::<HashSet<char>>();
        let common_items = unique_items_a.intersection(&unique_items_b);
        if verbose {
            println!("Common items in compartments {:?}", common_items);
        }

        let line_priority = calculate_priority(common_items);
        if verbose {
            println!("Line priority {}", line_priority);
        }

        total_priority += line_priority;
    }
    return (total_priority, -1);
}

fn split_into_compartments<'a>(line: &'a String) -> (&'a str, &'a str) {
    let mid = line.len() / 2;
    return (&line[0..mid], &line[mid..]);
}

fn calculate_priority<'a>(common_items: impl Iterator<Item = &'a char>) -> u32 {
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
