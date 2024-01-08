use utils::io::yield_lines_trimmed;

type ProbeDirection = u8;

#[derive(Debug)]
pub struct TreeGrid {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Tree>,
}

impl TreeGrid {
    pub fn get(&mut self, row: usize, col: usize) -> &mut Tree {
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
}

pub const NOT_VISIBLE: u8 = 0b0000;
pub const TOP_DOWN: u8 = 0b0001;
pub const BOTTOM_UP: u8 = 0b0010;
pub const LEFT_RIGHT: u8 = 0b0100;
pub const RIGHT_LEFT: u8 = 0b1000;

pub fn parse_tree_grid(file: &str) -> TreeGrid {
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

pub fn probe_trees(trees: &mut TreeGrid, direction: ProbeDirection, verbose: bool) {
    match direction {
        TOP_DOWN => {
            for col in 0..trees.width {
                let mut max_height: Option<usize> = None;
                for row in 0..trees.height {
                    let tree = trees.get(row, col);
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
                    let tree = trees.get(row, col);
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
                    let tree = trees.get(row, col);
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
                    let tree = trees.get(row, col);
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
