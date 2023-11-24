mod utils;

fn main() {
    let (input_file, verbose) = utils::parse_args();
    let (part_one, _) = solve(&input_file, verbose);

    println!("Part one: {}", part_one);
    // println!("Part two: {}", part_two);
}

fn solve(input_file: &str, verbose: bool) -> (u32, u32) {
    let mut fully_redundant_count = 0;
    for line in utils::yield_lines_trimmed(input_file) {
        let (range_a, range_b) = parse(line);
        if is_fully_redundant(&range_a, &range_b) {
            if verbose {
                println!("redundant! {range_a:?} {range_b:?}");
            }

            fully_redundant_count += 1;
        }
    }
    return (fully_redundant_count, 69);
}

fn parse(line: String) -> (Range, Range) {
    let comma = &line.find(',').expect("Could not find comma on line!");
    let (elve_1, elve_2) = (&line[..*comma], &line[*comma + 1..]);

    let elve_1_range = parse_elve_range(elve_1);
    let elve_2_range = parse_elve_range(elve_2);

    return (elve_1_range, elve_2_range);
}

fn parse_elve_range(range_to_parse: &str) -> Range {
    let dash = range_to_parse
        .find('-')
        .expect("Could not find dash on range!");
    let (start, end) = (&range_to_parse[..dash], &range_to_parse[dash + 1..]);
    return Range {
        start: start.parse::<u32>().expect("Could not parse number"),
        end: end.parse::<u32>().expect("Could not parse number"),
    };
}

fn is_fully_redundant(range_a: &Range, range_b: &Range) -> bool {
    range_a.is_inside(&range_b) || range_b.is_inside(&range_a)
}

#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn is_inside(&self, other: &Range) -> bool {
        &self.start >= &other.start && &self.end <= &other.end
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_part_one() {
        let (part_one_solved, _) = solve("./data_input.txt", false);
        assert_eq!(part_one_solved, 584);
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let (_, part_two_solved) = solve("./data_input.txt", false);
        assert_eq!(part_two_solved, 2716);
    }
}
