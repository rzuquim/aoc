#[derive(Debug)]
pub enum Instruction {
    Noop,
    Add { to_add: i32 },
}

impl Instruction {
    pub fn parse(line: String) -> Self {
        let mut split = line.split(' ');
        if let Some(instruction) = split.next() {
            match instruction {
                "addx" => {
                    if let Some(to_add_str) = split.next() {
                        let to_add = to_add_str.parse::<i32>().expect("number to add");
                        return Instruction::Add { to_add };
                    }
                }
                "noop" => return Instruction::Noop,
                _ => {}
            };
        }
        panic!("Could not read instruction from line {}", line);
    }

    pub fn cycles_for(&self) -> usize {
        return match self {
            Instruction::Noop => 1,
            Instruction::Add { to_add: _ } => 2,
        };
    }

    pub fn apply(&self, register: &mut i32) {
        match self {
            Instruction::Noop => {}
            Instruction::Add { to_add } => {
                *register += to_add;
            }
        };
    }
}
