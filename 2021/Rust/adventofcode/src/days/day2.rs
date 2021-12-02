use crate::shared::utils::{lines};
use std::str::FromStr;
use std::fs;

#[derive(Debug, PartialEq)]
enum Move {
    Forward,
    Down,
    Up
}

impl FromStr for Move {
    type Err = ();

    fn from_str(input: &str) -> Result<Move, Self::Err> {
        match input {
            "forward" => Ok(Move::Forward),
            "down" => Ok(Move::Down),
            "up" => Ok(Move::Up),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct MoveSteps {
    direction: Move,
    steps: i64
}

impl FromStr for MoveSteps {
    type Err = ();

    fn from_str(input: &str) -> Result<MoveSteps, Self::Err> {
        let parts: Vec<&str> = input.split(' ').collect();
        if parts.len() != 2 {
            return Err(());
        }

        let step = Move::from_str(parts[0]);
        if step.is_err() {
            return Err(());
        }

        let count = parts[1].parse::<i64>();
        if count.is_err() {
            return Err(());
        }

        Ok(MoveSteps { direction: step.unwrap(), steps: count.unwrap() })
    }
}

fn process(input: &str) {
    let lines = lines(input);

    let mut x = 0;
    let mut d = 0;
    let mut aim = 0;

    for line in lines {
        let move_steps = MoveSteps::from_str(&line);
        if move_steps.is_err() {
            panic!("Invalid Input {}" , line);
        }

        let move_steps = move_steps.unwrap();
        match move_steps.direction {
            Move::Forward => {
                d += move_steps.steps * aim;
                x += move_steps.steps;
            },
            Move::Down => aim += move_steps.steps,
            Move::Up => aim -= move_steps.steps,
        }
    }

    println!("Part 1 - {}", x * aim);
    println!("Part 2 - {}", x * d);
}

pub fn run() {
    let test_input = "
    forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2";

    println!("Test Input:");
    process(test_input);

    let input = fs::read_to_string("data/day2.txt").expect("Failed to read the file.");
    println!("Real Input:");
    process(&input);
}