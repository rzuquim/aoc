use crate::stack::CargoStack;
use crate::CrateCfg;

#[derive(Debug)]
pub struct MoveCmd {
    pub amount: usize,
    pub from: usize,
    pub to: usize,
}

impl MoveCmd {
    pub fn apply(&self, state: &mut Vec<CargoStack>, crate_cfg: &CrateCfg, verbose: bool) {
        let to_move = match crate_cfg {
            CrateCfg::CrateMover9000 => pop_one_by_one(self, state, verbose),
            CrateCfg::CrateMover9001 => pop_stack(self, state, verbose),
        };

        push(self, state, &to_move, verbose);
    }
}

fn pop_stack(move_cmd: &MoveCmd, state: &mut Vec<CargoStack>, verbose: bool) -> Vec<char> {
    let from_stack = state
        .get_mut(move_cmd.from)
        .expect(&format!("Could not find stack {}", move_cmd.from));
    let pop_index = from_stack.len() - move_cmd.amount;
    let popped = from_stack.drain(pop_index..).collect();
    if verbose {
        println!(
            "{:?} removed from {:?}! index {}",
            popped, from_stack, move_cmd.from
        );
    }
    return popped;
}

fn pop_one_by_one(move_cmd: &MoveCmd, state: &mut Vec<CargoStack>, verbose: bool) -> Vec<char> {
    let from_stack = state
        .get_mut(move_cmd.from)
        .expect(&format!("Could not find stack {}", move_cmd.from));
    let mut popped = Vec::with_capacity(move_cmd.amount);
    for _ in 0..move_cmd.amount {
        let le_crate = from_stack.pop().expect("Stack already empty!");
        popped.push(le_crate);
    }
    if verbose {
        println!(
            "{:?} removed from {:?}! index {}",
            popped, from_stack, move_cmd.from
        );
    }
    return popped;
}

fn push(move_cmd: &MoveCmd, state: &mut Vec<CargoStack>, to_push: &Vec<char>, verbose: bool) {
    let to_stack = state
        .get_mut(move_cmd.to)
        .expect(&format!("Could not find stack {}", move_cmd.to));
    for le_crate in to_push {
        to_stack.push(*le_crate);
    }
    if verbose {
        println!(
            "{:?} push into {:?}! index: {}",
            to_push, to_stack, move_cmd.to
        );
    }
}
