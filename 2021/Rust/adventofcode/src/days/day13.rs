use crate::day5::Point;
use crate::shared::utils::{lines, min_max};
use std::collections::HashMap;
use std::fs;
use std::iter;
use std::str::FromStr;

fn fold(points: &mut Vec<Point>, x: i16, y: i16) {
    for point in points.into_iter() {
        if point.x > x {
            point.x = 2 * x - point.x;
        }
        if point.y > y {
            // Fold at y=7 ==> 8 goes to 6, 9 to 5
            // 7 + (8 - point.y) = 2 * y + 1 - point.y
            point.y = 2 * y - point.y;
        }
    }

    let mut map = HashMap::new();
    let mut i = 0;
    while i < points.len() {
        let key = format!("{}_{}", points[i].x, points[i].y);
        if map.contains_key(&key) {
            points.remove(i);
        } else {
            map.insert(key, true);
            i += 1;
        }
    }

    print!("{} ", points.len());
}

fn process(input: &str) {
    let lines = lines(input);

    let mut points = Vec::new();
    for line in lines {
        if line.starts_with("fold along") {
            let is_x = &line[11..12] == "x";
            let val = line[13..].parse::<i16>().unwrap();
            fold(
                &mut points,
                if is_x { val } else { 10000 },
                if !is_x { val } else { 10000 },
            );
        } else {
            points.push(Point::from_str(&line).unwrap());
        }
    }
    println!();

    let x_range = min_max(points.iter().map(|&p| p.x)).unwrap();
    let y_range = min_max(points.iter().map(|&p| p.y)).unwrap();

    let mut vec_output: Vec<Vec<char>> = Vec::new();
    for _ in y_range.0..y_range.1 + 1 {
        let collect: Vec<char> = iter::repeat(' ')
            .take((x_range.1 - x_range.0 + 1) as usize)
            .collect();
        vec_output.push(collect);
    }

    for p in points {
        vec_output[(p.y - y_range.0) as usize][(p.x - x_range.0) as usize] = '#';
    }

    for l in vec_output {
        println!("{}", l.iter().collect::<String>());
    }
}

pub fn run() {
    let test_input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";
    println!("Test Input:");
    process(test_input);

    let input = fs::read_to_string("data/day13.txt").expect("Failed to read the file.");
    println!("Real Input:");
    process(&input);
}
