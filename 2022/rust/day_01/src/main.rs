use std::io::BufRead;
use std::{env, fs::File, io, path::Path};

fn main() {
    let (input_file, verbose) = parse_args();

    let path = Path::new(&input_file);
    let file = File::open(path).expect(format!("Could not open file {}", &input_file).as_str());
    let reader = io::BufReader::new(file);

    let mut max_calories = [0, 0, 0];
    let mut curr_acc = 0;
    let mut elve_index = 0;
    for line in reader.lines() {
        if let Some(calories) = parse_calories(line) {
            curr_acc = curr_acc + calories;
        } else {
            elve_index += 1;
            if let Some(position) = is_greater_than_previous_max(&curr_acc, &max_calories) {
                max_calories[position] = curr_acc;
            }
            if verbose {
                println!(
                    "Elve {} has {} calories. Current rank: {:?}",
                    elve_index, curr_acc, max_calories
                )
            }
            curr_acc = 0;
        }
    }

    let mut max_sum = 0;
    let mut max_max = 0;

    for max in max_calories {
        if max > max_max {
            max_max = max;
        }
        max_sum += max;
    }

    println!("Part one: {}", max_max);
    println!("Part two: {}", max_sum);
}

fn parse_calories(line: Result<String, std::io::Error>) -> Option<i32> {
    let contents = line.expect("Unexpected error reading line!");
    let trimmed = contents.trim();
    if trimmed.is_empty() {
        return None;
    }

    return Some(
        contents
            .parse::<i32>()
            .expect(format!("Could not parse line {} into calories count", contents).as_str()),
    );
}

fn is_greater_than_previous_max(curr_acc: &i32, max_calories: &[i32]) -> Option<usize> {
    let mut min_max = &i32::MAX;
    let mut min_idx = 0;

    // find out the minimum value on the collected max_calories
    for (i, curr_max) in max_calories.iter().enumerate() {
        if curr_max > min_max {
            continue;
        }

        min_max = curr_max;
        min_idx = i;
    }
    return if min_max > curr_acc {
        None
    } else {
        Some(min_idx)
    };
}

fn parse_args() -> (String, bool) {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file> <verbose flag?>", args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];
    if args.len() < 3 {
        return (input_file.clone(), false);
    }

    let verbose = match args[2].to_lowercase().as_str() {
        "true" | "-v" => true,
        "false" => false,
        _ => {
            eprintln!(
                "Unexpected verbose flag (use true, false or -v): '{}'",
                args[2]
            );
            std::process::exit(-1);
        }
    };

    return (input_file.clone(), verbose);
}
