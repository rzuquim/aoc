use utils::io::yield_lines_trimmed;

type ProbeDirection = u8;

#[derive(Debug)]
pub struct TreeGrid {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Tree>,
}

impl TreeGrid {
    pub fn get(&self, row: usize, col: usize) -> &Tree {
        let index = row * &self.width + col;
        return self
            .grid
            .get(index)
            .expect("Coordinates inside the grid bounds.");
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut Tree {
        let index = row * &self.width + col;
        return self
            .grid
            .get_mut(index)
            .expect("Coordinates inside the grid bounds.");
    }
}

#[derive(Debug)]
pub struct Tree {
    pub height: usize,
    pub visible_from: ProbeDirection,
    pub scenic_score: usize,
}

pub const NOT_VISIBLE: u8 = 0b0000;
pub const TOP_DOWN: u8 = 0b0001;
pub const BOTTOM_UP: u8 = 0b0010;
pub const LEFT_RIGHT: u8 = 0b0100;
pub const RIGHT_LEFT: u8 = 0b1000;

pub fn parse_tree_grid_from_file(file: &str) -> TreeGrid {
    let mut grid = Vec::<Tree>::new();
    let mut width = 0;
    let mut height = 0;

    for line in yield_lines_trimmed(file) {
        width = line.len();
        height += 1;
        for char in line.chars() {
            let tree = Tree {
                height: char.to_digit(10).expect("to be a number") as usize,
                visible_from: NOT_VISIBLE,
                scenic_score: 0,
            };
            grid.push(tree);
        }
    }

    return TreeGrid {
        width: width,
        height: height,
        grid: grid,
    };
}

pub fn calc_visibility_from_outside(
    trees: &mut TreeGrid,
    direction: ProbeDirection,
    verbose: bool,
) {
    match direction {
        TOP_DOWN => {
            for col in 0..trees.width {
                let mut max_height: Option<usize> = None;
                for row in 0..trees.height {
                    let tree = trees.get_mut(row, col);
                    max_height = mark_if_visible_and_update_max_height(tree, direction, max_height);
                    if verbose {
                        let is_visible = direction & tree.visible_from == direction;
                        println!(
                            "Tree ({}, {}) with height {} is visible TOP_DOWN? {}",
                            row, col, tree.height, is_visible
                        );
                    }
                }
            }
        }
        LEFT_RIGHT => {
            for row in 0..trees.height {
                let mut max_height: Option<usize> = None;
                for col in 0..trees.width {
                    let tree = trees.get_mut(row, col);
                    max_height = mark_if_visible_and_update_max_height(tree, direction, max_height);
                    if verbose {
                        let is_visible = direction & tree.visible_from == direction;
                        println!(
                            "Tree ({}, {}) with height {} is visible LEFT_RIGHT? {}",
                            row, col, tree.height, is_visible
                        );
                    }
                }
            }
        }
        BOTTOM_UP => {
            for col in 0..trees.width {
                let mut max_height = None;
                for row in (0..trees.height).rev() {
                    let tree = trees.get_mut(row, col);
                    max_height = mark_if_visible_and_update_max_height(tree, direction, max_height);
                    if verbose {
                        let is_visible = direction & tree.visible_from == direction;
                        println!(
                            "Tree ({}, {}) with height {} is visible BOTTOM_UP? {}",
                            row, col, tree.height, is_visible
                        );
                    }
                }
            }
        }
        RIGHT_LEFT => {
            for row in 0..trees.height {
                let mut max_height = None;
                for col in (0..trees.width).rev() {
                    let tree = trees.get_mut(row, col);
                    max_height = mark_if_visible_and_update_max_height(tree, direction, max_height);
                    if verbose {
                        let is_visible = direction & tree.visible_from == direction;
                        println!(
                            "Tree ({}, {}) with height {} is visible RIGHT_LEFT? {}",
                            row, col, tree.height, is_visible
                        );
                    }
                }
            }
        }
        _ => {
            panic!("Invalid direction! {}", direction)
        }
    };
}

pub fn calc_scenic_score(trees: &mut TreeGrid, verbose: bool) {
    for row in 0..trees.height {
        for col in 0..trees.width {
            set_scenic_score(row, col, trees, verbose);
        }
    }
}

pub fn set_scenic_score(row: usize, col: usize, trees: &mut TreeGrid, verbose: bool) {
    let up = get_scenic_core(row, col, trees, BOTTOM_UP, verbose);
    let down = get_scenic_core(row, col, trees, TOP_DOWN, verbose);
    let left = get_scenic_core(row, col, trees, RIGHT_LEFT, verbose);
    let right = get_scenic_core(row, col, trees, LEFT_RIGHT, verbose);

    let tree = trees.get_mut(row, col);
    tree.scenic_score = up * left * down * right;
    if verbose {
        println!(
            "Scenic score for ({}, {}) is ({} * {} * {} * {}) = {}",
            row, col, up, left, down, right, tree.scenic_score
        )
    }
}

fn get_scenic_core(
    row: usize,
    col: usize,
    trees: &TreeGrid,
    direction: ProbeDirection,
    verbose: bool,
) -> usize {
    let mut scenic_score = 0;
    let (row_inc, col_inc): (i32, i32) = match direction {
        BOTTOM_UP => (-1, 0),
        TOP_DOWN => (1, 0),
        LEFT_RIGHT => (0, 1),
        RIGHT_LEFT => (0, -1),
        _ => panic!("Invalid direction: {}", direction),
    };

    let from_tree = trees.get(row, col);
    let mut i: i32 = row as i32;
    let mut j: i32 = col as i32;
    loop {
        i += row_inc;
        j += col_inc;

        if out_of_grid_bounds(i, j, trees.width, trees.height) {
            break;
        }

        let neighbor = trees.get(i as usize, j as usize);
        scenic_score += 1;
        if neighbor.height >= from_tree.height {
            break; // view blocked
        }

        if verbose {
            println!(
                "Looking {} from ({}, {}) to ({}, {}) -> {}",
                direction, row, col, i, j, scenic_score
            )
        }
    }
    return scenic_score;
}

fn mark_if_visible_and_update_max_height(
    tree: &mut Tree,
    direction: ProbeDirection,
    max_height: Option<usize>,
) -> Option<usize> {
    if max_height == None || max_height.unwrap() < tree.height {
        tree.visible_from += direction;
        return Some(tree.height);
    }
    return max_height;
}

fn out_of_grid_bounds(row: i32, col: i32, grid_width: usize, grid_height: usize) -> bool {
    return row < 0 || col < 0 || row >= grid_height as i32 || col >= grid_width as i32;
}
