use std::env;
use std::io::BufRead;
use std::{fs::File, io, path::Path};

pub fn parse_args() -> (String, bool) {
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

pub fn yield_lines(file_path: &str) -> io::Lines<io::BufReader<File>> {
    let path = Path::new(file_path);
    let file = File::open(path).expect(format!("Could not open file {}", file_path).as_str());
    let reader = io::BufReader::new(file);
    return reader.lines();
}

pub fn yield_lines_trimmed(file_path: &str) -> impl Iterator<Item = String> {
  return yield_lines(file_path).map(|line| {
    let line = line.expect("Unexpected error reading line!");
    return line.trim().to_string()
  });
}
