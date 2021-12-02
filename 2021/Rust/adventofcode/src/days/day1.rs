use crate::shared::utils::{lines, lines_to_i64};
use std::fs;

fn positive_change(input: &Vec<i64>) -> i32
{
    let mut count = 0;
    for (i, v) in input.iter().enumerate() {
        if i == 0 {
            continue;
        }

        if input[i-1] < *v {
            count += 1;
        }
    }

    count
}

fn aggregate_blocks(input: &Vec<i64>) -> Vec<i64>
{
    let mut output = Vec::new();
    let mut total = 0;

    for (i, v) in input.iter().enumerate() {
        total += *v;

        if i < 2 {
            continue;
        }

        output.push(total);
        total -= input[i -2];
    }


    output
}

fn process(input: &str) {
    let ints = lines_to_i64(lines(&input));
    println!("{}", positive_change(&ints));
    let aggr = aggregate_blocks(&ints);
    println!("{}", positive_change(&aggr));
}

pub fn run() {
    let test_input = "199
200
208
210
200
207
240
269
260
263";

    println!("Test Input:");
    process(test_input);

    let input = fs::read_to_string("data/day1.txt").expect("Failed to read the file.");
    println!("Real Input:");
    process(&input);
}

