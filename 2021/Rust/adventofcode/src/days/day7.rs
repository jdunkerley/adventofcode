use std::cmp::{min, max};
use std::fs;

fn process(input: &str, cost: fn(i32) -> i32) {
    let positions: Vec<i16> = input.split(',').map(|x| x.parse::<i16>().unwrap()).collect();

    let range = positions.iter()
        .map(|x| (x, x))
        .reduce(|c, r| (min(c.0, r.0), max(c.1, r.1)))
        .unwrap();
    let range = *range.0..(*range.1 + 1);

    let mut best = (-1, -1);
    for v in range {
        let cost = positions.iter().map(|p| cost((p-v).abs() as i32)).sum();
        if cost < best.1 || best.0 == -1 {
            best = (v, cost);
        }
    }

    println!("{:?}", best);
}

pub fn run() {
    let test_input = "16,1,2,0,4,2,7,1,2,14";
    println!("Test Input:");
    process(test_input, |x| x);
    process(test_input, |x| x * (x + 1) / 2);

    let input = fs::read_to_string("data/day7.txt").expect("Failed to read the file.");
    println!("Real Input:");
    process(&input, |x| x);
    process(&input, |x| x * (x + 1) / 2);
}