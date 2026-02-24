use std::fs;

const TOTAL_CLICKS: u32 = 99;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Rotation {
    dir: Direction,
    clicks: u32,
}

fn parse_rotations(content: &str) -> Result<Vec<Rotation>, String> {
    content
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (first, last) = line.split_at(1);

            let dir = match first {
                "L" => Ok(Direction::Left),
                "R" => Ok(Direction::Right),
                other => Err(format!("unknown direction {}", other)),
            }?;

            let clicks = last
                .trim()
                .parse::<u32>()
                .map_err(|e| format!("invalid clicks: {e}"))?;
            Ok(Rotation { dir, clicks })
        })
        .collect()
}

fn apply_rotation(current: u32, rotation: &Rotation) -> u32 {
    match rotation.dir {
        Direction::Left => {
            let delta = rotation.clicks % (TOTAL_CLICKS + 1);
            return (current + (TOTAL_CLICKS + 1) - delta) % (TOTAL_CLICKS + 1);
        }
        Direction::Right => {
            return (current + rotation.clicks) % (TOTAL_CLICKS + 1);
        }
    }
}

fn find_password(rotations: Vec<Rotation>) -> u32 {
    let mut current: u32 = 50;
    let mut count: u32 = 0;

    for rotation in rotations.iter() {
        current = apply_rotation(current, rotation);
        // if current > 100 {
        //     println!("{}: {:?}", current, rotation);
        // }
        if current == 0 {
            count += 1;
        }
    }
    count
}

fn main() {
    let content = fs::read_to_string("./input.txt").expect("Failed to read file");

    let rotations = parse_rotations(&content).expect("Failed to parse rotations");
    let password = find_password(rotations);
    println!("Password: {}", password)
}
