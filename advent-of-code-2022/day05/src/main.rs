use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let (state_str, actions_str) = contents.split_once("\r\n\r\n").unwrap();

    let actions = parse_actions(actions_str);
    let mut state = parse_state(state_str);

    let result = execute_actions(&mut state, &actions);
    println!("The final state message is \"{result}\".");
}

#[derive(Debug, PartialEq)]
struct Action {
    amount: i32,
    from: u32,
    to: u32,
}

fn parse_state(state_str: &str) -> HashMap<u32, Vec<char>> {
    let mut parsed_state: Vec<Vec<char>> = state_str
        .split("\r\n")
        .map(|line| {
            let filtered_chars: Vec<char> = line
                .chars()
                .skip(1)
                .enumerate()
                .filter(|(i, _)| i % 4 == 0)
                .map(|(_, c)| c)
                .collect();
            filtered_chars
        })
        .collect();

    // Remove last row that contains column names.
    let labels = parsed_state
        .pop()
        .unwrap()
        .iter()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    let mut state: HashMap<u32, Vec<char>> = HashMap::new();
    for row in parsed_state.iter().rev() {
        for (label, elem) in labels.iter().zip(row.iter()) {
            if *elem != ' ' {
                match state.entry(*label) {
                    Entry::Vacant(e) => {
                        e.insert(vec![*elem]);
                    }
                    Entry::Occupied(mut e) => e.get_mut().push(*elem),
                }
            }
        }
    }

    state
}

fn parse_actions(actions_str: &str) -> Vec<Action> {
    let actions: Vec<Action> = actions_str
        .split("\r\n")
        .map(|line| {
            let words: Vec<&str> = line.split(" ").collect();
            let action = Action {
                amount: words[1].parse::<i32>().unwrap(),
                from: words[3].parse::<u32>().unwrap(),
                to: words[5].parse::<u32>().unwrap(),
            };

            action
        })
        .collect();

    actions
}

fn execute_actions(state: &mut HashMap<u32, Vec<char>>, actions: &Vec<Action>) -> String {
    for action in actions {
        let from_column = state.get_mut(&action.from).unwrap();

        let from_index = from_column.len() - action.amount as usize;
        let mut removed: Vec<char> = from_column.drain(from_index..).rev().collect();

        let to_column = state.get_mut(&action.to).unwrap();
        to_column.append(&mut removed);
    }

    let mut column_labels: Vec<u32> = state.keys().cloned().collect();
    column_labels.sort();

    // Get the top crates from each column.
    let result: String = column_labels
        .iter()
        .map(|k| match state.entry(*k) {
            Entry::Occupied(e) => e.get().last().unwrap_or(&' ').clone(),
            Entry::Vacant(_) => ' ',
        })
        .filter(|c| *c != ' ')
        .collect();

    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{execute_actions, parse_actions, parse_state, Action};

    #[test]
    fn test_parse_actions() {
        let actions_str = "move 1 from 2 to 1\r
move 3 from 1 to 3\r
move 2 from 2 to 1\r
move 1 from 1 to 2";

        let actions = parse_actions(actions_str);
        let expected = vec![
            Action {
                amount: 1,
                from: 2,
                to: 1,
            },
            Action {
                amount: 3,
                from: 1,
                to: 3,
            },
            Action {
                amount: 2,
                from: 2,
                to: 1,
            },
            Action {
                amount: 1,
                from: 1,
                to: 2,
            },
        ];

        for (a, b) in expected.iter().zip(actions.iter()) {
            assert_eq!(a, b);
        }
    }

    #[test]
    fn test_parse_state() {
        let state_str = "    [D]    \r
[N] [C]    \r
[Z] [M] [P]\r
 1   2   3 ";

        let state = parse_state(state_str);
        let expected = HashMap::from([
            (1, vec!['Z', 'N']),
            (2, vec!['M', 'C', 'D']),
            (3, vec!['P']),
        ]);

        for key in expected.keys() {
            let vec1 = state.get(key).unwrap();
            let vec2 = expected.get(key).unwrap();

            for (a, b) in vec2.iter().zip(vec1.iter()) {
                assert_eq!(a, b)
            }
        }
    }

    #[test]
    fn test_execute_actions() {
        let actions = vec![
            Action {
                amount: 1,
                from: 2,
                to: 1,
            },
            Action {
                amount: 3,
                from: 1,
                to: 3,
            },
            Action {
                amount: 2,
                from: 2,
                to: 1,
            },
            Action {
                amount: 1,
                from: 1,
                to: 2,
            },
        ];

        let mut state = HashMap::from([
            (1, vec!['Z', 'N']),
            (2, vec!['M', 'C', 'D']),
            (3, vec!['P']),
        ]);

        let result = execute_actions(&mut state, &actions);
        assert_eq!("CMZ", result);
    }
}
