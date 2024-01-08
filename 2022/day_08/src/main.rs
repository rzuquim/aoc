use day_08::*;

fn main() {
    let (input_file, verbose) = utils::io::parse_args();
    let part_one = solve(&input_file, verbose);

    println!("Part one: {:?}", part_one);
    // println!("Part two: {:?}", part_two);
}

fn solve(file: &str, verbose: bool) -> usize {
    let mut trees = parse_tree_grid(file);

    probe_trees(&mut trees, TOP_DOWN, verbose);
    probe_trees(&mut trees, LEFT_RIGHT, verbose);
    probe_trees(&mut trees, BOTTOM_UP, verbose);
    probe_trees(&mut trees, RIGHT_LEFT, verbose);

    if verbose {
        let visible: Vec<&Tree> = trees.grid.iter().filter(|t| t.visible_from > 0).collect();
        println!("visible trees: {:?}", visible);
        let hidden: Vec<&Tree> = trees.grid.iter().filter(|t| t.visible_from == 0).collect();
        println!("hidden trees: {:?}", hidden);
    }

    return trees.grid.iter().filter(|t| t.visible_from > 0).count();
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part_one() {
        let part_one = solve("./data/input.txt", false);
        assert_eq!(part_one, 1849);
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        todo!();
        // let dirs_sizes = calc_dirs_sizes("./data_input.txt", false);
        // let part_two = solve_part_two(&dirs_sizes, 70000000, 30000000, false);
        // assert_eq!(part_two, 4978279);
    }
}
