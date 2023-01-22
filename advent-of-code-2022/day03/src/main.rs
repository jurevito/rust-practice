use std::collections::{HashMap, HashSet};
use std::fs;
use std::str::Chars;

fn main() {
    let contents = fs::read_to_string("./src/input.txt").unwrap();

    // Create scoring map.
    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

    let result = contents
        .split("\r\n")
        .flat_map(|sack| {
            let cutoff = sack.len() / 2;
            let part1: Chars = sack[..cutoff].chars();
            let part2: Chars = sack[cutoff..].chars();

            let set1: HashSet<char> = HashSet::from_iter(part1);
            let set2: HashSet<char> = HashSet::from_iter(part2);

            let scores = set1
                .intersection(&set2)
                .cloned()
                .map(|c| letter_scores.get(&c).unwrap())
                .collect::<Vec<&usize>>();
            scores
        })
        .sum::<usize>();

    println!("Sum of priorities is {result}.");
}
