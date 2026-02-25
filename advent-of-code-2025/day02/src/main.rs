use std::fs;

#[derive(Debug, Copy, Clone)]
struct Range {
    first_id: u64,
    last_id: u64,
}

fn parse_ranges(content: &str) -> Result<Vec<Range>, String> {
    content
        .split(",")
        .map(|elem| {
            let (first_str, last_str) = elem.split_once("-").ok_or("split range string")?;

            let first_id = first_str
                .trim()
                .parse::<u64>()
                .map_err(|e| format!("parse first id: {e}"))?;

            let last_id = last_str
                .trim()
                .parse::<u64>()
                .map_err(|e| format!("parse last id: {e}"))?;

            Ok(Range { first_id, last_id })
        })
        .collect()
}

fn is_invalid(id: u64) -> bool {
    let s: Vec<char> = id.to_string().chars().collect();
    if s.len() % 2 == 1 {
        return false;
    }

    for i in 0..s.len() / 2 {
        if s[i] != s[s.len() / 2 + i] {
            return false;
        }
    }

    true
}

fn sum_invalid_ids(id_range: Range) -> u64 {
    (id_range.first_id..=id_range.last_id)
        .filter(|&id| is_invalid(id))
        .map(|id| id as u64)
        .sum()
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("Failed to read file");
    let id_ranges = parse_ranges(&content).expect("Failed to parse id ranges");

    let sum: u64 = id_ranges
        .iter()
        .map(|&id_range| sum_invalid_ids(id_range))
        .sum();

    println!("Sum: {}", sum);
}
