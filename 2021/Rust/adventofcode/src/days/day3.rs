use crate::shared::utils::{lines};
use std::fs;

fn more_ones(lines: &Vec<String>, index: usize) -> bool {
    let mut count: u16 = 0;

    for line in lines {
        if line.chars().nth(index).unwrap() == '1' {
            count += 1;
        }
    }

    let length = lines.len() as u16;
    count * 2 >= length
}

fn filter_list(lines: &Vec<String>, highest: bool) -> u32 {
    let mut filtered = lines.clone();
    let mut index: usize = 0;
    while filtered.len() > 1 {
        let char = if more_ones(&filtered, index) { '1' } else { '0' };
        if highest {
            filtered.retain(|x| x.chars().nth(index).unwrap() == char);
        } else {
            filtered.retain(|x| x.chars().nth(index).unwrap() != char);
        }
        index += 1;
    }

    u32::from_str_radix(&filtered[0], 2).unwrap()
}

fn process(input: &str) {
    let lines = lines(input);

    let length = lines[0].chars().count();

    let mut counts : Vec<u16> = Vec::with_capacity(length);
    for _i in 0..length {
        counts.push(0);
    }

    for line in &lines {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                counts[i] += 1;
            }
        }
    }

    let threshold = (lines.len() / 2) as u16;
    let mut output = String::from("");
    for i in 0..length {
        output += if counts[i] > threshold { "1" } else { "0" };
    }

    let value = u32::from_str_radix(&output, 2).unwrap();
    let value2 = 2_u32.pow(length as u32) - 1 - value;

    println!("{}", value * value2);
    println!("{}", filter_list(&lines, true) * filter_list(&lines, false));
}

pub fn run() {
    let test_input = "
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    println!("Test Input:");
    process(test_input);

    let input = fs::read_to_string("data/day3.txt").expect("Failed to read the file.");
    println!("Real Input:");
    process(&input);
}