mod utils;

fn main() {
    let (input_file, verbose) = utils::parse_args();
    let (part_one, part_two) = solve(&input_file, verbose);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}

fn solve(input_file: &str, verbose: bool) -> (i32, i32) {
    let mut part_one = 0;
    let mut part_two = 0;
    for line in utils::yield_lines(input_file) {
        let (round_pt_one, round_pt_two) = parse_game_rounds(line);
        let curr_round_points_pt_one = round_points(&round_pt_one);
        let curr_round_points_pt_two = round_points(&round_pt_two);
        if verbose {
            print_round(&round_pt_one, &curr_round_points_pt_one, "Part 1");
            print_round(&round_pt_two, &curr_round_points_pt_two, "Part 2");
        }
        part_one += curr_round_points_pt_one;
        part_two += curr_round_points_pt_two;
    }
    return (part_one, part_two);
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

fn parse_game_rounds(line: Result<String, std::io::Error>) -> (Round, Round) {
    let line = line.expect("Unexpected error reading line!");
    let line = line.trim();
    let line = line.chars().collect();

    return (strategy_part_one(&line), strategy_part_two(&line));
}

fn strategy_part_one(line: &Vec<char>) -> Round {
    let opponents_choice = parse_players_choice(&line, 0);
    let my_choice = parse_players_choice(&line, 2);
    let round_result = calc_round_result(&my_choice, &opponents_choice);

    return Round {
        my_choice,
        opononents_choice: opponents_choice,
        result: round_result,
    };
}

fn strategy_part_two(line: &Vec<char>) -> Round {
    let opponents_choice = parse_players_choice(&line, 0);
    let round_result = parse_round_result(&line, 2);
    let my_choice = calc_my_choice(&opponents_choice, &round_result);

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

fn parse_round_result(line: &Vec<char>, char_idx: usize) -> RoundResult {
    let letter = line[char_idx];
    return match &letter {
        'X' => RoundResult::Loose,
        'Y' => RoundResult::Draw,
        'Z' => RoundResult::Win,
        _ => panic!("Unexpected value parsing round resutl: {}", letter),
    };
}

fn calc_round_result(my_choice: &PlayersChoice, opponents_choice: &PlayersChoice) -> RoundResult {
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

fn calc_my_choice(opponents_choice: &PlayersChoice, round_result: &RoundResult) -> PlayersChoice {
    match (opponents_choice, round_result) {
        (PlayersChoice::Rock, RoundResult::Loose) => PlayersChoice::Scissors,
        (PlayersChoice::Rock, RoundResult::Draw) => PlayersChoice::Rock,
        (PlayersChoice::Rock, RoundResult::Win) => PlayersChoice::Paper,
        (PlayersChoice::Paper, RoundResult::Loose) => PlayersChoice::Rock,
        (PlayersChoice::Paper, RoundResult::Draw) => PlayersChoice::Paper,
        (PlayersChoice::Paper, RoundResult::Win) => PlayersChoice::Scissors,
        (PlayersChoice::Scissors, RoundResult::Loose) => PlayersChoice::Paper,
        (PlayersChoice::Scissors, RoundResult::Draw) => PlayersChoice::Scissors,
        (PlayersChoice::Scissors, RoundResult::Win) => PlayersChoice::Rock,
    }
}

fn print_round(r: &Round, points: &i32, part: &str) {
    println!(
        "{}: {:?} vs {:?} - {:?} ({} points)",
        part, r.my_choice, r.opononents_choice, r.result, points
    );
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

    #[test]
    fn test_part_two() {
        let (_, part_two_solved) = solve("./data_input.txt", false);
        assert_eq!(part_two_solved, 16862);
    }
}
