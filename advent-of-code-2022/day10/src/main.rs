use std::fs;

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let instructions = parse_instructions(&contents);
    let sum_strength = sum_signal_strengths(instructions);
    
    println!("Sum of signal strengths: {}.", sum_strength);
}

fn sum_signal_strengths(instructions: Vec<Instruction>) -> i32 {
    let mut cycle = 0;
    let mut register = 1;
    let mut signal_strengths: Vec<i32> = Vec::with_capacity(16);

    for instruct in instructions {

        let n_cycles = match instruct {
            Instruction::Addx(_) => 2,
            Instruction::Noop => 1,
        };

        for _ in 0..n_cycles {
            cycle += 1;

            if (cycle - 20) % 40 == 0 {
                signal_strengths.push(cycle*register);
            }
        }

        if let Instruction::Addx(value) = instruct {
            register += value;
        }
    }

    signal_strengths.iter().sum::<i32>()
}

fn parse_instructions(contents: &str) -> Vec<Instruction> {
    contents
        .split("\r\n")
        .map(|line| {
            match line.split_once(" ") {
                Some((_, value)) => Instruction::Addx(value.parse::<i32>().expect("Cannot parse value as integer.")),
                _ => Instruction::Noop,
            }
        })
        .collect()
}