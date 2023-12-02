pub type CargoStack = Vec<char>;

pub trait CargoStackTrait {
    fn peek(&self) -> Option<&char>;
    fn insert_bottom(&mut self, char: char);
}

impl CargoStackTrait for CargoStack {
    fn peek(&self) -> Option<&char> {
        self.last()
    }

    fn insert_bottom(&mut self, char: char) {
        self.insert(0, char)
    }
}
