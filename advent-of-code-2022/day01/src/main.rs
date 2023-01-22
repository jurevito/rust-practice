use std::fs;

fn main() {
    let contents = fs::read_to_string("./src/input.txt").expect("Cannot open the file.");

    let calories: Vec<i32> = contents
        .split("\r\n\r\n")
        .map(|sub| sub.split("\r\n")
            .map(|s| s.parse::<i32>().unwrap())
            .sum::<i32>())
        .collect();

    let (index, &max_cals) = calories.iter()
        .enumerate()
        .max_by_key(|t| t.1)
        .unwrap_or((0, &0));

    println!("Elf {index} has {max_cals} calories.");
}
