use crate::move_cmd::MoveCmd;
use crate::stack::{CargoStack, CargoStackTrait};

pub fn initial_state(line: String, state: &mut Vec<CargoStack>, verbose: bool) -> bool {
    if line.is_empty() {
        return false;
    }

    let mut curr_stack_idx = 0;
    let mut curr_stack = get_or_create_stack(state, curr_stack_idx);
    for (i, char) in line.chars().enumerate() {
        if i / 4 > curr_stack_idx {
            curr_stack_idx = i / 4;
            curr_stack = get_or_create_stack(state, curr_stack_idx);
        }

        match CharOnState::parse(char) {
            CharOnState::Meaningless => continue,
            CharOnState::EndOfInitialStateDescription => {
                return false;
            }
            CharOnState::Crate => {
                if verbose {
                    println!("push crate {char} on stack {curr_stack_idx}");
                }
                curr_stack.insert_bottom(char);
            }
        }
    }
    return true;
}

pub fn move_cmd(line: String, verbose: bool) -> MoveCmd {
    let split: Vec<_> = line.split(' ').collect();
    let move_cmd = MoveCmd {
        amount: split[1].parse::<usize>().expect("Could not parse 'amount'"),
        from: split[3].parse::<usize>().expect("Could not parse 'from'") - 1,
        to: split[5].parse::<usize>().expect("Could not parse 'to'") - 1,
    };
    if verbose {
        println!("{move_cmd:?}")
    }
    return move_cmd;
}

fn get_or_create_stack<'a>(state: &'a mut Vec<CargoStack>, stack_idx: usize) -> &'a mut CargoStack {
    if let None = state.get(stack_idx) {
        let stack = CargoStack::new();
        state.push(stack);
    }
    return &mut state[stack_idx];
}

enum CharOnState {
    Meaningless,
    Crate,
    EndOfInitialStateDescription,
}

impl CharOnState {
    fn parse(char: char) -> CharOnState {
        match char {
            'A'..='Z' => CharOnState::Crate,
            '1' => CharOnState::EndOfInitialStateDescription,
            ' ' | '[' | ']' => CharOnState::Meaningless,
            _ => panic!("Could not parse {char} as valid state"),
        }
    }
}
