use std::fs;

fn main() {
    let contents = fs::read_to_string("./src/input.txt").expect("Cannot open the file.");

    let score: i32 = contents
        .split("\r\n")
        .map(|s| {
            let chars = s.chars().collect::<Vec<char>>();
            let hand_score = match chars[2] {
                'X' => 1,
                'Y' => 2,
                'Z' => 3,
                _ => 0,
            };
            
            let outcome_score = match (chars[0], chars[2]) {
                ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
                ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
                _ => 0,
            };

            outcome_score + hand_score
        }).sum();

    println!("Score is {score}.");
}