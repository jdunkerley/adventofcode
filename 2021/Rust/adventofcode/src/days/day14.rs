use crate::shared::utils::{lines, min_max};
use std::collections::HashMap;
use std::fs;

fn run_step(
    state: &HashMap<[char; 2], usize>,
    map: &HashMap<[char; 2], char>,
) -> HashMap<[char; 2], usize> {
    let mut output = HashMap::new();

    for (k, count) in state {
        let v = map.get(k).unwrap();

        let k1 = [k[0], *v];
        output.insert(k1, output.get(&k1).unwrap_or(&0_usize) + count);

        let k2 = [*v, k[1]];
        output.insert(k2, output.get(&k2).unwrap_or(&0_usize) + count);
    }

    output
}

fn report(state: &HashMap<[char; 2], usize>, last: char) {
    let mut counts = HashMap::new();
    counts.insert(last, 1);
    for (key, count) in state.iter() {
        let v0 = counts.get(&key[0]).unwrap_or(&0_i64) + *count as i64;
        counts.insert(key[0], v0);
    }

    let (min, max) = min_max(counts.values().into_iter()).unwrap();
    println!("{} : {:?}", max - min, counts);
}

fn process(input: &str) {
    let lines = lines(input);

    let mut map = HashMap::new();
    for line in lines.iter().skip(1) {
        let temp: Vec<char> = line.chars().collect();
        let k = [temp[0], temp[1]];
        let v = temp[6];
        map.insert(k, v);
    }

    let template: Vec<char> = lines[0].chars().collect();
    let mut state: HashMap<[char; 2], usize> = HashMap::new();
    for i in 0..template.len() - 1 {
        let k = [template[i], template[i + 1]];
        state.insert(k, state.get(&k).unwrap_or(&0_usize) + 1);
    }

    for _ in 0..40 {
        state = run_step(&state, &map);
        report(&state, *template.last().unwrap());
    }
}

pub fn run() {
    let test_input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    println!("Test Input:");
    process(test_input);

    let input = fs::read_to_string("data/day14.txt").expect("Failed to read the file.");
    println!("Real Input:");
    process(&input);
}
