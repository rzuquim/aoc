use std::collections::HashSet;

use day_09::*;
use utils::io::yield_lines_trimmed;

type Coord = (i32, i32);

fn main() {
    let (input_file, verbose) = utils::io::parse_args();
    let part_one = solve(&input_file, 1, verbose);
    println!("Part one: {:?}", part_one);
    let part_two = solve(&input_file, 9, verbose);
    println!("Part two: {:?}", part_two);
}

pub fn solve(input_file: &str, knots_count: usize, verbose: bool) -> usize {
    let mut head = RopeKnot::new();
    let mut knots = (0..knots_count)
        .map(|_| RopeKnot::new())
        .collect::<Vec<RopeKnot>>();
    let mut tail_visited_positions = HashSet::<(i32, i32)>::new();

    // starting position
    tail_visited_positions.insert((0, 0));

    for line in yield_lines_trimmed(&input_file) {
        let (direction, count) = parse_move_cmd(line);
        apply(
            direction,
            count,
            &mut head,
            &mut knots,
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
    knots: &mut Vec<RopeKnot>,
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

        let tail_position = advance_knots(knots, head.x, head.y);
        tail_history.insert(tail_position);

        if verbose {
            for (i, knot) in knots.iter().enumerate() {
                println!("Knot {}: ({}, {})", i, knot.x, knot.y);
            }
            println!();
        }
    }
}

fn advance_knots(knots: &mut Vec<RopeKnot>, head_x: i32, head_y: i32) -> Coord {
    let mut prev_knot = (head_x, head_y);
    for knot in knots {
        let (is_separated, (dx, dy)) = calc_distance(&prev_knot, knot);
        if !is_separated {
            prev_knot = (knot.x, knot.y);
            continue;
        }

        let (dx, dy) = follow(dx, dy);
        knot.x += dx;
        knot.y += dy;
        prev_knot = (knot.x, knot.y);
    }
    return prev_knot; // not moved
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part_one() {
        let part_one = solve("./data/input.txt", 1, false);
        assert_eq!(part_one, 6498);
    }

    #[test]
    fn test_part_two() {
        let part_two = solve("./data/input.txt", 9, false);
        assert_eq!(part_two, 2531);
    }
}
