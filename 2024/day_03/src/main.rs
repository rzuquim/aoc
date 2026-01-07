mod interpreter;
mod tokenizer;

use tokenizer::Tokenizer;

use utils::io::open_read_buffer;

use crate::interpreter::Interpreter;

fn main() {
    let (input_file, verbose) = utils::io::parse_args();
    let part_one = eval_mul_instructions(&input_file, verbose);
    println!("Part one: {:?}", part_one);
    // let part_two = solve_part_2(&input_file, verbose);
    // println!("Part two {:?}", part_two);
}

fn eval_mul_instructions(file: &str, verbose: bool) -> u32 {
    let read = open_read_buffer(file);
    let tokenizr = Tokenizer::new(read, verbose);
    let mut interpreter = Interpreter::new(tokenizr);

    let mut result = 0;
    while let Some(instruction) = interpreter.next() {
        if let Ok(instruction) = instruction {
            let i_result = instruction.run();
            result += i_result;
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part_one() {
        let part_one = eval_mul_instructions("./data/input.txt", false);
        assert_eq!(part_one, 220);
    }

    // #[test]
    // fn test_part_two() {
    //     let part_one = solve_part_2("./data/input.txt", false);
    //     assert_eq!(part_one, 296);
    // }
}
