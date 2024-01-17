type Coord = (i32, i32);

pub struct RopeKnot {
    pub x: i32,
    pub y: i32,
}

impl RopeKnot {
    pub fn new() -> Self {
        return Self { x: 0, y: 0 };
    }
}

pub fn parse_move_cmd(line: String) -> (char, u32) {
    let mut chars = line.split_whitespace();
    let letter: char;
    let count: u32;

    if let Some(direction_string) = chars.next() {
        letter = direction_string.chars().next().expect("Invalid direction");
    } else {
        panic!("Could not read direction from line: {}.", line)
    }

    if let Some(count_string) = chars.next() {
        count = count_string
            .parse::<u32>()
            .expect("Invalid movement count!");
    } else {
        panic!("Could not read movement count from line: {}.", line)
    }
    return (letter, count);
}

pub fn calc_distance(prev_knot: &Coord, knot: &RopeKnot) -> (bool, Coord) {
    let dx = prev_knot.0 - knot.x;
    let dy = prev_knot.1 - knot.y;
    // using square to remove signal
    let should_follow = dx * dx > 1 || dy * dy > 1;
    return (should_follow, (dx, dy));
}

pub fn follow(x: i32, y: i32) -> Coord {
    return (x.signum(), y.signum());
}
