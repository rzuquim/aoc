use utils::io::yield_lines;

fn main() {
    let (input_file, verbose) = utils::io::parse_args();
    let part_one = count_safe_reports(&input_file, verbose);
    println!("Part one: {:?}", part_one);
    // let part_two = solve_part_2(&input_file, verbose);
    // println!("Part two: {:?}", part_two);
}

pub fn count_safe_reports(input_file: &str, verbose: bool) -> usize {
    return yield_lines(&input_file)
        .enumerate()
        .filter(|(i, report_line)| is_safe_report(report_line.as_ref().unwrap(), i, verbose))
        .count();
}

fn is_safe_report(line: &str, i: &usize, verbose: bool) -> bool {
    if verbose {
        println!("Report {}", i)
    }

    let mut last_lvl = None;
    let mut report_direction = None;

    for level_str in line.split(' ') {
        let level = level_str.parse::<i32>().unwrap();
        if last_lvl.is_none() {
            last_lvl = Some(level);
            continue;
        }

        let last_lvl_value = last_lvl.unwrap();
        let diff = last_lvl_value - level;

        if diff == 0 {
            if verbose {
                println!("\tUNSAFE REPORT {}: nor asc nor desc {}", i, diff);
            }
            return false;
        }

        if diff.pow(2) > MAX_ALLOWED_STEP_SQUARED {
            if verbose {
                println!("\tUNSAFE REPORT {}: big step {}", i, diff);
            }
            return false;
        }

        let level_direction = if level > last_lvl_value {
            Dir::Asc
        } else {
            Dir::Desc
        };

        if report_direction.is_none() {
            report_direction = Some(level_direction.clone());
        }

        if verbose {
            println!(
                "\tLevel changed from {} -> {}: ({:?} {})",
                last_lvl_value, level, level_direction, diff
            )
        }

        if level_direction.diverge(&report_direction) {
            if verbose {
                println!("\tUNSAFE REPORT {}: changed direction", i);
            }
            return false;
        }

        last_lvl = Some(level);
    }
    return true;
}

#[derive(Debug, Clone)]
enum Dir {
    Asc,
    Desc,
}

impl Dir {
    fn diverge(&self, other: &Option<Dir>) -> bool {
        return matches!(
            (self, other.as_ref().unwrap()),
            (Dir::Asc, Dir::Desc) | (Dir::Desc, Dir::Asc)
        );
    }
}

// NOTE: avoiding sqrt
const MAX_ALLOWED_STEP_SQUARED: i32 = 3 * 3;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part_one() {
        let part_one = count_safe_reports("./data/input.txt", false);
        assert_eq!(part_one, 220);
    }
}
