mod utils;

fn main() {
    let (input_file, verbose) = utils::parse_args();
    let (part_one, part_two) = solve(&input_file, verbose);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}

fn solve(input_file: &str, verbose: bool) -> (i32, i32) {
    let mut part_one = 0;
    for line in utils::yield_lines(input_file) {
        let round = parse_game_round(line);
        let current_round_points = round_points(&round);
        if verbose {
            println!(
                "{:?} vs {:?} - {:?} ({} points)",
                round.my_choice, round.opononents_choice, round.result, current_round_points
            );
        }
        part_one += current_round_points;
    }
    return (part_one, -1);
}

fn round_points(round: &Round) -> i32 {
    let choice_points = match round.my_choice {
        PlayersChoice::Rock => 1,
        PlayersChoice::Paper => 2,
        PlayersChoice::Scissors => 3,
    };

    let result_points = match round.result {
        RoundResult::Win => 6,
        RoundResult::Draw => 3,
        RoundResult::Loose => 0,
    };

    return choice_points + result_points;
}

fn parse_game_round(line: Result<String, std::io::Error>) -> Round {
    let line = line.expect("Unexpected error reading line!");
    let line = line.trim();
    let line = line.chars().collect();

    let opponents_choice = parse_players_choice(&line, 0);
    let my_choice = parse_players_choice(&line, 2);
    let round_result = round_result(&my_choice, &opponents_choice);

    return Round {
        my_choice,
        opononents_choice: opponents_choice,
        result: round_result,
    };
}

fn parse_players_choice(line: &Vec<char>, char_idx: usize) -> PlayersChoice {
    let letter = line[char_idx];
    return match &letter {
        'A' | 'X' => PlayersChoice::Rock,
        'B' | 'Y' => PlayersChoice::Paper,
        'C' | 'Z' => PlayersChoice::Scissors,
        _ => panic!("Unexpected value parsing players choice: {}", letter),
    };
}

fn round_result(my_choice: &PlayersChoice, opponents_choice: &PlayersChoice) -> RoundResult {
    match (my_choice, opponents_choice) {
        (PlayersChoice::Rock, PlayersChoice::Paper) => RoundResult::Loose,
        (PlayersChoice::Rock, PlayersChoice::Scissors) => RoundResult::Win,
        (PlayersChoice::Paper, PlayersChoice::Scissors) => RoundResult::Loose,
        (PlayersChoice::Paper, PlayersChoice::Rock) => RoundResult::Win,
        (PlayersChoice::Scissors, PlayersChoice::Rock) => RoundResult::Loose,
        (PlayersChoice::Scissors, PlayersChoice::Paper) => RoundResult::Win,
        _ => RoundResult::Draw,
    }
}

#[derive(Debug)]
struct Round {
    my_choice: PlayersChoice,
    opononents_choice: PlayersChoice,
    result: RoundResult,
}

#[derive(Debug)]
enum RoundResult {
    Win,
    Draw,
    Loose,
}

#[derive(Debug)]
enum PlayersChoice {
    Rock,
    Paper,
    Scissors,
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_part_one() {
        let (part_one_solved, _) = solve("./data_input.txt", false);
        assert_eq!(part_one_solved, 11475);
    }
}
