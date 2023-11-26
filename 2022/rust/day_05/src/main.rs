mod move_cmd;
mod parse;
mod utils;

use utils::io;
use utils::stack::{CargoStack, CargoStackTrait};

fn main() {
    let (input_file, verbose) = io::parse_args();
    let (part_one, part_two) = solve(&input_file, verbose);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}

fn solve(input_file: &str, verbose: bool) -> (String, String) {
    let mut crates_state = Vec::<CargoStack>::new();
    let mut parsing_initial_state = true;

    for line in io::yield_lines(input_file) {
        let line = line.expect("Could not read line");
        if line.is_empty() {
            continue;
        }

        if parsing_initial_state {
            parsing_initial_state = parse::initial_state(line, &mut crates_state, verbose);
            if verbose && !parsing_initial_state {
                print_initial_state(&crates_state);
            }
            continue;
        }

        let move_cmd = parse::move_cmd(line, verbose);
        move_cmd.apply(&mut crates_state, verbose)
    }

    let last_state = crates_state.iter().map(|stack| {
        if let Some(top) = stack.peek() {
            top
        } else {
            &' '
        }
    });

    return (String::from_iter(last_state), String::from("TODO"));
}

fn print_initial_state(state: &Vec<CargoStack>) {
    for (i, stack) in state.iter().enumerate() {
        println!("Stack {stack_num}: {stack:?}", stack_num = i + 1);
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_part_one() {
        let (part_one_solved, _) = solve("./data_input.txt", false);
        assert_eq!(part_one_solved, "RNZLFZSJH");
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let (_, part_two_solved) = solve("./data_input.txt", false);
        assert_eq!(part_two_solved, "abcde");
    }
}
