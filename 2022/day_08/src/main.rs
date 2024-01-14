use day_08::*;

fn main() {
    let (input_file, verbose) = utils::io::parse_args();
    let trees = parse_tree_grid(&input_file, verbose);

    let part_one = solve_part_one(&trees);
    println!("Part one: {:?}", part_one);

    let part_two = solve_part_two(&trees);
    println!("Part two: {:?}", part_two);
}

fn parse_tree_grid(file: &str, verbose: bool) -> TreeGrid {
    let mut trees = parse_tree_grid_from_file(file);

    calc_visibility_from_outside(&mut trees, TOP_DOWN, verbose);
    calc_visibility_from_outside(&mut trees, LEFT_RIGHT, verbose);
    calc_visibility_from_outside(&mut trees, BOTTOM_UP, verbose);
    calc_visibility_from_outside(&mut trees, RIGHT_LEFT, verbose);

    calc_scenic_score(&mut trees, verbose);

    if verbose {
        let visible: Vec<&Tree> = trees.grid.iter().filter(|t| t.visible_from > 0).collect();
        println!("visible trees: {:?}", visible);
        let hidden: Vec<&Tree> = trees.grid.iter().filter(|t| t.visible_from == 0).collect();
        println!("hidden trees: {:?}", hidden);
    }
    return trees;
}

fn solve_part_one(trees: &TreeGrid) -> usize {
    return trees.grid.iter().filter(|t| t.visible_from > 0).count();
}

fn solve_part_two(trees: &TreeGrid) -> usize {
    return trees.grid.iter().map(|t| t.scenic_score).max().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part_one() {
        let trees = parse_tree_grid("./data/input.txt", false);
        let part_one = solve_part_one(&trees);
        assert_eq!(part_one, 1849);
    }

    #[test]
    fn test_part_two() {
        let trees = parse_tree_grid("./data/input.txt", false);
        let part_two = solve_part_two(&trees);
        assert_eq!(part_two, 201600);
    }
}
