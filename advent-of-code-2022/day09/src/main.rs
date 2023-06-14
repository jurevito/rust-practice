use std::{fs, collections::HashMap};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    amount: u32,
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let cmds = parse_commands(&contents);
    
    let n_visited = n_positions(&cmds);
    println!("Number of visited positions: {}.", n_visited);
}

fn n_positions(cmds: &Vec<Command>) -> usize {
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);
    
    let mut visited: HashMap<(i32, i32), bool> = HashMap::new();
    visited.insert(tail, true);

    for cmd in cmds {
        for _ in 0..cmd.amount {
            head = match cmd.direction {
                Direction::Up => (head.0, head.1 + 1),
                Direction::Down => (head.0, head.1 - 1),
                Direction::Right => (head.0 + 1, head.1),
                Direction::Left => (head.0 - 1, head.1),
            };

            let diff = ((head.0 - tail.0), (head.1 - tail.1));
            tail = match (diff.0.abs(), diff.1.abs()) {
                (2, _) => (tail.0 + diff.0 / 2, tail.1 + diff.1),
                (_, 2) => (tail.0 + diff.0, tail.1 + diff.1 / 2),
                _ => tail,
            };

            visited.insert(tail, true);
        }
    }

    visited.len()
}

fn parse_commands(contents: &str) -> Vec<Command> {
    contents
        .split("\r\n")
        .map(|line| {
            match line.split_once(" ") {
                Some((dir, amount)) => {

                    let cmd = Command {
                        direction: match dir {
                            "U" => Direction::Up,
                            "D" => Direction::Down,
                            "R" => Direction::Right,
                            "L" => Direction::Left,
                            _ => panic!("Direction is not valid."),
                        },
                        amount: amount.parse::<u32>().expect("Amount is not in correct format."),
                    };
                    
                    cmd
                },
                _ => panic!("Error while parsing."),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use crate::{parse_commands, n_positions};

    #[test]
    fn number_of_visited() {
        let contents = "R 4\r
U 4\r
L 3\r
D 1\r
R 4\r
D 1\r
L 5\r
R 2";

        let cmds = parse_commands(&contents);
        let n_visited = n_positions(&cmds);

        assert_eq!(13, n_visited);
    }
}