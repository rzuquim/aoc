use std::collections::HashSet;

use day_09::*;
use utils::io::yield_lines_trimmed;

fn main() {
    let (input_file, verbose) = utils::io::parse_args();
    let part_one = solve_part_one(&input_file, verbose);
    println!("Part one: {:?}", part_one);
}

pub fn solve_part_one(input_file: &str, verbose: bool) -> usize {
    let mut head = RopeKnot::new();
    let mut tail = RopeKnot::new();
    let mut tail_visited_positions = HashSet::<(i32, i32)>::new();

    // starting position
    tail_visited_positions.insert((tail.x, tail.y));

    for line in yield_lines_trimmed(&input_file) {
        let (direction, count) = parse_move_cmd(line);
        apply(
            direction,
            count,
            &mut head,
            &mut tail,
            &mut tail_visited_positions,
            verbose,
        );
    }

    return tail_visited_positions.len();
}

pub fn apply(
    direction: char,
    count: u32,
    head: &mut RopeKnot,
    tail: &mut RopeKnot,
    tail_history: &mut HashSet<(i32, i32)>,
    verbose: bool,
) {
    let move_cmd = match direction {
        'U' => (0, 1),
        'D' => (0, -1),
        'L' => (-1, 0),
        'R' => (1, 0),
        _ => panic!("Invalid direction!"),
    };

    for _ in 0..count {
        head.x += move_cmd.0;
        head.y += move_cmd.1;

        if verbose {
            println!(
                "Moving head {}{} -> ({}, {})",
                direction, count, head.x, head.y
            );
        }

        let (is_separated, (dx, dy)) = calc_distance(head, tail);
        if verbose {
            println!(
                "Distance between ({}, {}) and ({}, {}): ({}, {})",
                head.x, head.y, tail.x, tail.y, dx, dy
            );
        }

        if !is_separated {
            continue;
        }

        let (dx, dy) = tail_follows(dx, dy);

        tail.x += dx;
        tail.y += dy;

        if verbose {
            println!("Tail followed! ({}, {})", tail.x, tail.y);
        }
        tail_history.insert((tail.x, tail.y));
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part_one() {
        let part_one = solve_part_one("./data/input.txt", false);
        assert_eq!(part_one, 6498);
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        // let part_two = solve_part_two("./data/input.txt", false);
        // assert_eq!(part_two, 6498);
        todo!()
    }
}
