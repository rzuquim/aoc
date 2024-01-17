use std::collections::HashMap;

use day_10::*;
use utils::io::yield_lines_trimmed;

type ImportantCycle = usize;
type RegisterValue = i32;

fn main() {
    let (input_file, verbose) = utils::io::parse_args();
    let part_one = solve(&input_file, verbose);
    println!("Part one: {:?}", part_one);
    // let part_two = solve(&input_file, 9, verbose);
    // println!("Part two: {:?}", part_two);
}

pub fn solve(input_file: &str, verbose: bool) -> i32 {
    let mut register: i32 = 1;
    let mut cycle: usize = 0;

    let important_cycles = [20, 60, 100, 140, 180, 220];
    let mut important_cycle_idx = 0;
    let mut register_values_on_important = HashMap::<ImportantCycle, RegisterValue>::new();

    'instructions: for instruction in yield_lines_trimmed(input_file).map(Instruction::parse) {
        let cycles_cost = instruction.cycles_for();

        for _ in 0..cycles_cost {
            cycle += 1;
            if important_cycles[important_cycle_idx] != cycle {
                continue;
            }

            register_values_on_important.insert(cycle, register);

            important_cycle_idx += 1;
            if important_cycle_idx >= important_cycles.len() {
                break 'instructions;
            }
        }
        instruction.apply(&mut register);
        if verbose {
            println!(
                "{:?} cycles: {}, register: {}",
                &instruction, cycle, register
            );
        }
    }

    if verbose {
        println!("{:?}", register_values_on_important);
    }

    return register_values_on_important
        .iter()
        .map(|(&k, &v)| k as i32 * v)
        .sum();
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part_one() {
        let part_one = solve("./data/input.txt", false);
        assert_eq!(part_one, 12980);
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        //let part_two = solve("./data/input.txt", 9, false);
        //assert_eq!(part_two, 2531);
        todo!()
    }
}
