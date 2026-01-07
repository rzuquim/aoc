use std::io::BufRead;

// NOTE: on a real implementation it would be better to tokenize over a char iterator
pub struct Tokenizer<R: BufRead> {
    file: R,
    file_buf: Vec<u8>,
    buf_pos: usize,
    /// line position, char on line
    file_pos: (usize, usize),
    evaluating: Vec<char>,
    verbose: bool,
}

impl<R: BufRead> Tokenizer<R> {
    pub fn new(file: R, verbose: bool) -> Self {
        return Tokenizer {
            file,
            file_buf: Vec::new(),
            buf_pos: 0,
            file_pos: (0, 0),
            evaluating: Vec::new(),
            verbose,
        };
    }
}

#[derive(Debug)]
pub struct TokenizerResult {
    pub token: Token,
    pub token_pos: (usize, usize),
    pub giberish: String,
    pub giberish_pos: (usize, usize),
}

#[derive(Debug)]
pub enum Token {
    Mul,
    Number(u32),
    LParen,
    RParen,
    Comma,
    LineBreak,
}

impl Token {
    fn len(&self) -> usize {
        return match self {
            Token::Mul => 3,
            Token::LParen => 1,
            Token::RParen => 1,
            Token::Comma => 1,
            Token::LineBreak => 1,
            Token::Number(number) => {
                let mut n = *number;
                let mut len = 1;
                while n >= 10 {
                    n = n / 10;
                    len += 1;
                }
                return len;
            }
        };
    }
}

impl<R: BufRead> Iterator for Tokenizer<R> {
    type Item = Result<TokenizerResult, String>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.file_buf.len() <= self.buf_pos {
                let length = self.file_buf.len();
                self.file.consume(length);

                self.file_buf = match self.file.fill_buf() {
                    Ok(buf) => buf,
                    Err(e) => return Some(Err(e.to_string())),
                };
            }

            if self.file_buf.is_empty() {
                return None;
            }

            while self.buf_pos < self.file_buf.len() {
                // NOTE: not handling UTF-8
                let c = self.file_buf[self.buf_pos] as char;
                self.buf_pos += 1;
                self.evaluating.push(c);

                let token = check_token(&self.evaluating);

                let Some(token) = token else {
                    continue;
                };

                let found_line_break = matches!(&token, Token::LineBreak);

                let token_len = token.len();
                let eval_len = self.evaluating.len();
                let token_pos = token_pos(token_len, eval_len, self.file_pos);
                let giberish = giberish(token_len, &self.evaluating);
                let found_token = TokenizerResult {
                    token,
                    token_pos,
                    giberish: String::from_iter(giberish),
                    giberish_pos: self.file_pos,
                };

                if self.verbose {
                    println!("Found token {found_token:?}");
                }

                if found_line_break {
                    // NOTE: going to next line
                    self.file_pos = (0, self.file_pos.1 + 1);
                } else {
                    self.file_pos.0 += eval_len;
                }

                self.evaluating.clear();
                return Some(Ok(found_token));
            }
        }
    }
}

fn giberish(token_len: usize, evaluating: &Vec<char>) -> Vec<char> {
    let split_pos = evaluating.len() - token_len;
    if split_pos <= 0 {
        return Vec::new();
    };

    let (giberish, _) = evaluating.split_at(split_pos);
    return giberish.to_vec();
}

fn token_pos(token_len: usize, eval_len: usize, file_pos: (usize, usize)) -> (usize, usize) {
    let token_start_pos = file_pos.0 + eval_len - token_len;
    return (token_start_pos, file_pos.1);
}

fn check_token(evaluating: &[char]) -> Option<Token> {
    if evaluating.is_empty() {
        return None;
    }

    let eval_len = evaluating.len();
    let last_char = evaluating[eval_len - 1];
    match last_char {
        ',' => return Some(Token::Comma),
        '\n' => return Some(Token::LineBreak),
        '(' => return Some(Token::LParen),
        ')' => return Some(Token::RParen),
        _ => (),
    };

    if eval_len >= 3 {
        let slice = &evaluating[eval_len - 3..eval_len];
        if slice.iter().copied().eq("mul".chars()) {
            return Some(Token::Mul);
        }
    }

    // TODO: try not to allocate this string
    let number = evaluating.iter().collect::<String>().parse::<u32>();
    if let Ok(number) = number {
        return Some(Token::Number(number));
    }

    return None;
}
