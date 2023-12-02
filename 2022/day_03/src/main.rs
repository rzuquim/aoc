use std::collections::HashSet;

use utils::io;

fn main() {
    let (input_file, verbose) = io::parse_args();
    let (part_one, part_two) = solve(&input_file, verbose);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}

fn solve(input_file: &str, verbose: bool) -> (u32, u32) {
    let mut priority_by_commonality = 0; // part one
    let mut priority_by_badge = 0; // part two

    let mut curr_elve_group = Vec::<Rucksack>::with_capacity(3);

    for line in io::yield_lines_trimmed(input_file) {
        let rucksack = parse_rucksack(line, verbose);

        // part one
        let common_items = common_items_in_compartments(&rucksack, verbose);
        let line_priority = calculate_priority_by_commonality(common_items, verbose);
        priority_by_commonality += line_priority;

        // part two
        curr_elve_group.push(rucksack);
        if curr_elve_group.len() == 3 {
            let badge_letter = find_out_badge(&curr_elve_group, verbose);
            priority_by_badge += item_priority(&badge_letter);
            curr_elve_group.clear();
        }
    }
    return (priority_by_commonality, priority_by_badge);
}

fn parse_rucksack(line: String, verbose: bool) -> Rucksack {
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

fn common_items_in_compartments(
    rucksack: &Rucksack,
    verbose: bool,
) -> impl Iterator<Item = &char> {
    let common_items = rucksack.compartment_a.intersection(&rucksack.compartment_b);
    if verbose {
        println!("Common items in compartments {:?}", common_items);
    }
    return common_items;
}

fn find_out_badge(rucksacks: &Vec<Rucksack>, verbose: bool) -> char {
    let first_rucksack = rucksacks.first().unwrap();
    let mut common_items = first_rucksack.complete_rucksack();

    // intersection with other two
    for rucksack in rucksacks.iter().skip(1) {
        common_items = common_items
            .intersection(&rucksack.complete_rucksack())
            .cloned()
            .collect::<HashSet<char>>();
    }

    if common_items.len() > 1 {
        panic!("Could not find the common item on the rucksacks!");
    }

    let badge_letter = *common_items.iter().next().unwrap();
    if verbose {
        println!("Common item in elves rucksack: {badge_letter}!");
    }
    return badge_letter;
}

fn calculate_priority_by_commonality<'a>(
    common_items: impl Iterator<Item = &'a char>,
    verbose: bool,
) -> u32 {
    let priority = common_items.map(item_priority).sum();
    if verbose {
        println!("Line priority {}", priority);
    }
    return priority;
}

fn item_priority(item: &char) -> u32 {
    if item.is_lowercase() {
        (*item as u32) - ('a' as u32) + 1
    } else {
        (*item as u32) - ('A' as u32) + 27
    }
}

struct Rucksack {
    compartment_a: HashSet<char>,
    compartment_b: HashSet<char>,
}

impl Rucksack {
    fn complete_rucksack(&self) -> HashSet<char> {
        (&self.compartment_a)
            .union(&self.compartment_b)
            .map(|i| *i)
            .collect::<HashSet<char>>()
    }
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
    fn test_part_two() {
        let (_, part_two_solved) = solve("./data_input.txt", false);
        assert_eq!(part_two_solved, 2716);
    }
}
