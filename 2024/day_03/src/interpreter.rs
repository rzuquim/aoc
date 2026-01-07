use crate::tokenizer::{Token, Tokenizer};
use std::io::BufRead;

pub struct Interpreter<R: BufRead> {
    tokenizr: Tokenizer<R>,
}

impl<R: BufRead> Interpreter<R> {
    pub fn new(tokenizr: Tokenizer<R>) -> Self {
        return Self { tokenizr };
    }
}

impl<R: BufRead> Iterator for Interpreter<R> {
    type Item = Result<Instr, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut curr_instr: Option<InstrExpectation> = None;
        let mut params: Vec<u32> = Vec::new();

        while let Some(token_result) = self.tokenizr.next() {
            let token_result = match token_result {
                Ok(t) => t,
                Err(e) => return Some(Err(e)),
            };

            // TODO: print giberish found

            let token = token_result.token;
            if let Some(new_instr) = is_valid_instruction(&token) {
                if curr_instr.is_some() {
                    let err_msg = format!(
                        "instruction already found. got {:?} and then {:?}",
                        curr_instr, token
                    );
                    return Some(Err(err_msg));
                }

                curr_instr = Some(new_instr);
                continue;
            }

            // NOTE: got a valid token before the instruction. eg. a number one before a `mul`
            let Some(ref instruction) = curr_instr else {
                let err_msg = format!("valid instruction expected. got {:?}", token);
                return Some(Err(err_msg));
            };

            if let Some(number) = is_number(&token) {
                params.push(number);
                continue;
            };

            if instruction.ready(&params) {
                return Some(instruction.build(&params));
            }
        }

        // NOTE: the last instruction must
        if let Some(ref instruction) = curr_instr {
            if instruction.ready(&params) {
                return Some(instruction.build(&params));
            } else {
                let err_msg = format!("incomplete trailing instruction: {:?}", instruction);
                return Some(Err(err_msg));
            }
        }

        return None;
    }
}

fn is_number(token: &Token) -> Option<u32> {
    return match token {
        Token::Number(number) => Some(*number),
        _ => None,
    };
}

fn is_valid_instruction(token: &Token) -> Option<InstrExpectation> {
    return match token {
        Token::Mul => Some(InstrExpectation::Multiplication),
        _ => None,
    };
}

pub enum Instr {
    Multiplication(u32, u32),
}

impl Instr {
    pub fn run(&self) -> u32 {
        return match self {
            Instr::Multiplication(a, b) => a * b,
        };
    }
}

#[derive(Debug)]
pub enum InstrExpectation {
    Multiplication,
}

impl InstrExpectation {
    fn ready(&self, params: &[u32]) -> bool {
        return match self {
            InstrExpectation::Multiplication => params.len() == 2,
        };
    }

    fn build(&self, params: &[u32]) -> Result<Instr, String> {
        return match self {
            InstrExpectation::Multiplication => {
                if params.len() == 2 {
                    Ok(Instr::Multiplication(params[0], params[1]))
                } else {
                    Err("Multiplication expects 2 parameters".to_string())
                }
            }
        };
    }
}
