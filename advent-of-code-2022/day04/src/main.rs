use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let n_redundant = number_of_redundant(&contents);

    println!("In {n_redundant} instance one range fully contains the other.");
}

fn number_of_redundant(contents: &str) -> i32 {
    let score = contents
        .split("\r\n")
        .map(|line| {
            // Parse line into vector of ranges.
            let ranges = line
                .split(",")
                .map(|entry| {
                    let numbers: Vec<i32> = entry
                        .split("-")
                        .map(|n| n.parse::<i32>().unwrap_or(0))
                        .collect();

                    let range = (numbers[0], numbers[1]);
                    range
                })
                .collect::<Vec<(i32, i32)>>();

            // Check if one contains the other.
            let ((x1, x2), (y1, y2)) = (ranges[0], ranges[1]);
            ((x1 >= y1 && x2 <= y2) || (x1 <= y1 && x2 >= y2)) as i32
        })
        .sum::<i32>();

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_results() {
        let contents = "2-4,6-15\r
20-30,40-50\r
5-7,7-9\r
2-8,3-7\r
6-6,4-6\r
2-6,4-12";

        assert_eq!(2, number_of_redundant(contents));
    }
}
