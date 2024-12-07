use std::fs;

#[derive(Debug)]
struct Equation {
    test: i32,
    numbers: Vec<i32>,
}

enum Operator {
    Add,
    Multiply,
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("Should be able to read the file");
    let mut result = 0;

    for line in content.lines() {
        if line.trim().len() > 0 {
            if let Some((test_str, numbers_str)) = line.split_once(": ") {
                let mut eq = Equation {
                    test: test_str.trim().parse().unwrap(),
                    numbers: numbers_str.split(" ").map(|n| n.trim().parse().unwrap()).collect(),
                };

                if is_calibrated(&mut eq) {
                    result += eq.test;
                }

                println!("{:?}", eq);
                
                
            }
        }
    }

    println!("Calibration result: {}", result); // correct 3749
}

fn is_calibrated(eq: &mut Equation) -> bool {
    if eq.numbers.len() == 0 {
        return eq.test == 0;
    }

    let mut scores: Vec<i32> = vec![eq.numbers[0]];
    let mut index: usize = 1;
    while index < eq.numbers.len() {
        let mut new_scores: Vec<i32> = vec![];
    
        for score in &scores {
            let add_score = score + eq.numbers[index];
            let mult_score = score * eq.numbers[index];
            if add_score == eq.test || mult_score == eq.test {
                return true;
            }

            new_scores.push(add_score);
            new_scores.push(mult_score);
        }

        scores = new_scores;
        index += 1;
    }

    return false;
}
