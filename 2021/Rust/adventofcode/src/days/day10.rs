use crate::shared::utils::lines;
use std::fs;

fn check_line(line: &str) -> (bool, i64) {
    let mut stack = Vec::new();

    for c in line.chars() {
        match c {
            '[' => stack.push(']'),
            '(' => stack.push(')'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            ']' | ')' | '}' | '>' => {
                let d = stack.pop();
                if d.is_some() && d.unwrap() == c {
                    continue;
                }

                return (
                    false,
                    match c {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => panic!("Unexpected closing character"),
                    },
                );
            }
            _ => panic!("Unexpected character {}", c),
        }
    }

    stack.reverse();
    (
        true,
        stack
            .iter()
            .map(|c| match *c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!("Unexpected closing character"),
            })
            .reduce(|a, b| a * 5 + b)
            .unwrap(),
    )
}

fn process(input: &str) {
    let checks: Vec<(bool, i64)> = lines(input).iter().map(|x| check_line(x)).collect();
    println!(
        "Error Check {}",
        checks
            .iter()
            .filter(|a| a.0 == false)
            .map(|a| a.1)
            .sum::<i64>()
    );

    let mut autocorrect: Vec<i64> = checks.iter().filter(|b| b.0).map(|b| b.1).collect();
    autocorrect.sort_by_key(|v| *v);
    println!("Auto correct: {}", autocorrect[autocorrect.len() / 2]);
}

pub fn run() {
    let test_input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
    println!("Test Input:");
    process(test_input);

    let input = fs::read_to_string("data/day10.txt").expect("Failed to read the file.");
    println!("Real Input:");
    process(&input);
}
