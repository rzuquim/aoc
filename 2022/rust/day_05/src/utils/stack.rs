#[derive(Debug)]
pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    pub fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    pub fn peek(&self) -> Option<&T> {
        self.stack.last()
    }

    /// adds the items
    pub fn setup(&mut self, item: T) {
        self.stack.insert(0, item);
    }
}
