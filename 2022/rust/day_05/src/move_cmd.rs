use crate::utils::stack::Stack;

#[derive(Debug)]
pub struct MoveCmd {
    pub amount: usize,
    pub from: usize,
    pub to: usize,
}

impl MoveCmd {
    pub fn pop(&self, state: &mut Vec<Stack<char>>, verbose: bool) -> Vec<char> {
        let from_stack = state
            .get_mut(self.from)
            .expect(&format!("Could not find stack {}", self.from));
        let mut popped = Vec::with_capacity(self.amount);
        for _ in 0..self.amount {
            let le_crate = from_stack.pop().expect("Stack already empty!");
            popped.push(le_crate);
        }
        if verbose {
            println!(
                "{:?} removed from {:?}! index {}",
                popped, from_stack, self.from
            );
        }
        return popped;
    }

    pub fn push(&self, state: &mut Vec<Stack<char>>, to_push: &Vec<char>, verbose: bool) {
        let to_stack = state
            .get_mut(self.to)
            .expect(&format!("Could not find stack {}", self.to));
        for le_crate in to_push {
            to_stack.push(*le_crate);
        }
        if verbose {
            println!("{:?} push into {:?}! index: {}", to_push, to_stack, self.to);
        }
    }
}
