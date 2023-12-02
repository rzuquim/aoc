use utils::io;

fn main() {
    let (input_file, verbose) = io::parse_args();
    let (part_one, part_two) = solve(&input_file, verbose);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}

fn solve(input_file: &str, verbose: bool) -> (i32, i32) {
    let mut max_calories = [0, 0, 0];
    let mut curr_acc = 0;
    let mut elve_index = 0;

    for line in io::yield_lines(input_file) {
        if let Some(calories) = parse_calories(line) {
            curr_acc = curr_acc + calories;
        } else {
            elve_index += 1;
            if let Some(position) = is_greater_than_previous_max(&curr_acc, &max_calories) {
                max_calories[position] = curr_acc;
            }
            if verbose {
                println!(
                    "Elve {} has {} calories. Current rank: {:?}",
                    elve_index, curr_acc, max_calories
                )
            }
            curr_acc = 0;
        }
    }

    let part_one_solution = max_calories.iter().max().unwrap();
    let part_two_solution = max_calories.iter().sum();

    return (*part_one_solution, part_two_solution);
}

fn parse_calories(line: Result<String, std::io::Error>) -> Option<i32> {
    let contents = line.expect("Unexpected error reading line!");
    let trimmed = contents.trim();
    if trimmed.is_empty() {
        return None;
    }

    return Some(
        contents
            .parse::<i32>()
            .expect(format!("Could not parse line {} into calories count", contents).as_str()),
    );
}

fn is_greater_than_previous_max(curr_acc: &i32, max_calories: &[i32]) -> Option<usize> {
    // find out the minimum value on the collected max_calories
    let (min_index, min_max) = max_calories
        .iter()
        .enumerate()
        .min_by_key(|&(_, &value)| value)
        .expect("max_calories should not be empty");

    return if min_max > curr_acc {
        None
    } else {
        Some(min_index)
    };
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_part_one() {
        let (part_one_solved, _) = solve("./data_input.txt", false);
        assert_eq!(part_one_solved, 66487)
    }

    #[test]
    fn test_part_two() {
        let (_, part_two_solved) = solve("./data_input.txt", false);
        assert_eq!(part_two_solved, 197301)
    }
}
