use crate::shared::utils::lines;
use std::fs;

// 1 __c__f_ 2 0
// 7 a_c__f_ 3 1
// 4 _bcd_f_ 4 2
// 5 ab_d_fg 5 3
// 2 a_cde_g 5 4
// 3 a_cd_fg 5 5
// 6 ab_defg 6 6
// 0 abc_efg 6 7
// 9 abcd_fg 6 8
// 8 abcdefg 7 9

fn count_matches(a: &str, b: &str) -> usize {
    a.chars()
        .map(|c| if b.contains(&String::from(c)) { 1 } else { 0 })
        .sum()
}

fn process_line(line: &str) -> (i32, i32) {
    let index = line.find(" | ");
    if index.is_none() {
        panic!("Bad format line {}", line);
    }
    let index = index.unwrap();

    let mut codes: Vec<&str> = line[0..index].split(' ').collect();
    codes.sort_by(|a, b| a.len().cmp(&b.len()));

    let mut ordered = codes.clone();
    ordered[1] = codes[0];
    ordered[7] = codes[1];
    ordered[4] = codes[2];
    ordered[8] = codes[9];

    // Lets find 3
    if count_matches(codes[3], ordered[1]) == 2 {
        ordered[3] = codes[3];
        codes[3] = "";
    } else if count_matches(codes[4], ordered[1]) == 2 {
        ordered[3] = codes[4];
        codes[4] = "";
    } else if count_matches(codes[5], ordered[1]) == 2 {
        ordered[3] = codes[5];
        codes[5] = "";
    }

    // Identify b
    let b = String::from(
        ordered[4]
            .chars()
            .filter(|&c| !ordered[3].contains(&String::from(c)))
            .collect::<Vec<char>>()[0],
    );

    // Lets find 5
    if count_matches(codes[3], &b) == 1 {
        ordered[5] = codes[3];
        codes[3] = "";
    } else if count_matches(codes[4], &b) == 1 {
        ordered[5] = codes[4];
        codes[4] = "";
    } else if count_matches(codes[5], &b) == 1 {
        ordered[5] = codes[5];
        codes[5] = "";
    }

    // 2 is the left over 5
    ordered[2] = if codes[3] != "" {
        codes[3]
    } else if codes[4] != "" {
        codes[4]
    } else {
        codes[5]
    };

    // Lets find 9
    if count_matches(codes[6], ordered[4]) == 4 {
        ordered[9] = codes[6];
        codes[6] = "";
    } else if count_matches(codes[7], ordered[4]) == 4 {
        ordered[9] = codes[7];
        codes[7] = "";
    } else if count_matches(codes[8], ordered[4]) == 4 {
        ordered[9] = codes[8];
        codes[8] = "";
    }

    // Lets find 6
    if count_matches(codes[6], ordered[1]) == 1 {
        ordered[6] = codes[6];
        codes[6] = "";
    } else if count_matches(codes[7], ordered[1]) == 1 {
        ordered[6] = codes[7];
        codes[7] = "";
    } else if count_matches(codes[8], ordered[1]) == 1 {
        ordered[6] = codes[8];
        codes[8] = "";
    }

    // 0 is the left over 6
    ordered[0] = if codes[6] != "" {
        codes[6]
    } else if codes[7] != "" {
        codes[7]
    } else {
        codes[8]
    };

    fn match_position(to_match: &str, ordered: &Vec<&str>) -> i32 {
        let len = to_match.len();
        for (i, code) in ordered.iter().enumerate() {
            if code.len() == len && count_matches(to_match, code) == len {
                // println!("{} {} {} {:?}", to_match, i, code, ordered);
                return i as i32;
            }
        }
        panic!("Could not match {} in {:?}", to_match, ordered);
    }

    let output: Vec<i32> = line[index + 3..]
        .split(' ')
        .map(|c| match_position(c, &ordered))
        .collect();
    let output = output[0] * 1000 + output[1] * 100 + output[2] * 10 + output[3];

    (
        line[index + 3..]
            .split(' ')
            .filter(|x| x.len() < 5 || x.len() == 7)
            .count() as i32,
        output,
    )
}

fn process(input: &str) {
    let lines: Vec<String> = lines(&input);
    println!(
        "{:?}",
        lines
            .iter()
            .map(|x| process_line(x))
            .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
            .unwrap()
    );
}

pub fn run() {
    let test_input = "
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
    println!("Test Input:");
    process(test_input);

    let input = fs::read_to_string("data/day8.txt").expect("Failed to read the file.");
    println!("Real Input:");
    process(&input);
}
