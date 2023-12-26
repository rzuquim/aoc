mod io;
mod marker_detector;

use io::{advance_to_next_line, fill_buffer, next_sequence};
use marker_detector::detect_marker;
use utils::io::open_read_buffer;

fn main() {
    let (input_file, verbose) = utils::io::parse_args();
    let part_one = solve(&input_file, 4, verbose);
    let part_two = solve(&input_file, 14, verbose);

    println!("Part one: {:?}", part_one);
    println!("Part two: {:?}", part_two);
}

fn solve(input_file: &str, window_size: usize, verbose: bool) -> Vec<usize> {
    let mut marker_per_line = Vec::<usize>::new();

    let mut buffer = vec![0; window_size];

    let mut reader = open_read_buffer(input_file);
    let mut starting_line = true;
    let mut curr_position = 0;

    loop {
        if starting_line {
            starting_line = false;
            let file_is_over = fill_buffer(&mut buffer, &mut reader);
            if file_is_over {
                if marker_per_line.len() == 0 {
                    panic!("Could not find any markers!");
                } else {
                    break;
                }
            }
        } else {
            curr_position += 1;
            next_sequence(&mut buffer, &mut reader);
        }

        let marker_detected = detect_marker(&buffer);
        if verbose {
            println!("Marker detected on {:?}: {}", &buffer, marker_detected);
        }

        if marker_detected {
            marker_per_line.push(curr_position + buffer.len());
            if !advance_to_next_line(&mut reader) {
                break;
            }
            starting_line = true;
            curr_position = 0;
            continue;
        }
    }

    return marker_per_line;
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_part_one() {
        let part_one_solved = solve("./data_input.txt", 4, false);
        assert_eq!(
            *part_one_solved.first().expect("Could not find marker"),
            1538
        );
    }

    #[test]
    fn test_part_two() {
        let part_two_solved = solve("./data_input.txt", 14, false);
        assert_eq!(
            *part_two_solved.first().expect("Could not find marker"),
            2315
        );
    }
}
