use crate::shared::utils::lines;
use std::collections::HashMap;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Point {
    x: i16,
    y: i16,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(input: &str) -> Result<Point, Self::Err> {
        let parts: Vec<Result<i16, ParseIntError>> =
            input.split(',').map(|x| x.parse::<i16>()).collect();

        if parts.len() == 2 && parts.iter().all(|x| x.is_ok()) {
            Ok(Point {
                x: *parts[0].as_ref().unwrap(),
                y: *parts[1].as_ref().unwrap(),
            })
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Line {
    start: Point,
    end: Point,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(input: &str) -> Result<Line, Self::Err> {
        let index = input.find(" -> ");
        match index {
            None => Err(()),
            Some(i) => {
                let start = Point::from_str(&input[0..i]);
                let end = Point::from_str(&input[i + 4..]);
                if start.is_ok() && end.is_ok() {
                    Ok(Line {
                        start: start.unwrap(),
                        end: end.unwrap(),
                    })
                } else {
                    Err(())
                }
            }
        }
    }
}

impl Line {
    fn is_horizontal(self: &Self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(self: &Self) -> bool {
        self.start.x == self.end.x
    }

    fn step(self: &Self) -> (i16, i16) {
        if self.is_vertical() {
            (0, if self.end.y > self.start.y { 1 } else { -1 })
        } else if self.is_horizontal() {
            (if self.end.x > self.start.x { 1 } else { -1 }, 0)
        } else {
            (
                if self.end.x > self.start.x { 1 } else { -1 },
                if self.end.y > self.start.y { 1 } else { -1 },
            )
        }
    }

    fn len(self: &Self) -> usize {
        if self.is_vertical() {
            (self.end.y - self.start.y).abs() as usize
        } else {
            (self.end.x - self.start.x).abs() as usize
        }
    }

    fn point(self: &Self, step: usize) -> Point {
        Point {
            x: self.start.x + self.step().0 * step as i16,
            y: self.start.y + self.step().1 * step as i16,
        }
    }
}

impl IntoIterator for Line {
    type Item = Point;
    type IntoIter = LineIter;

    fn into_iter(self) -> Self::IntoIter {
        LineIter {
            data: self,
            step: 0,
        }
    }
}

struct LineIter {
    data: Line,
    step: usize,
}

impl Iterator for LineIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.step <= self.data.len() {
            self.step += 1;
            Some(self.data.point(self.step - 1))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_parse() {
        let point_result = Point::from_str("123,456");
        assert!(point_result.is_ok());
        let point = point_result.unwrap();
        assert_eq!(point.x, 123);
        assert_eq!(point.y, 456);
    }

    #[test]
    fn test_line_parse() {
        let line_result = Line::from_str("424,253 -> 56,253");
        assert!(line_result.is_ok());

        let line = line_result.unwrap();
        assert_eq!(line.start.x, 424);
        assert_eq!(line.start.y, 253);
        assert_eq!(line.end.x, 56);
        assert_eq!(line.end.y, 253);
    }
}

fn get_overlaps<'a, I>(lines: I) -> usize
where
    I: IntoIterator<Item = &'a Line>,
{
    let mut grid = HashMap::new();
    for line in lines {
        for point in line.into_iter() {
            let current = grid.remove(&point);
            grid.insert(
                point,
                if current.is_none() {
                    1_u8
                } else {
                    current.unwrap() + 1
                },
            );
        }
    }

    grid.iter().filter(|kvp| *kvp.1 > 1).count()
}

fn process(input: &str) {
    let lines: Vec<Line> = lines(&input)
        .iter()
        .map(|x| Line::from_str(x).unwrap())
        .collect();

    println!(
        "H/V {}",
        get_overlaps(
            lines
                .iter()
                .filter(|x| x.is_horizontal() || x.is_vertical())
        )
    );
    println!("All {}", get_overlaps(lines.iter().filter(|_| true)));
}

pub fn run() {
    let test_input = "
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

    println!("Test Input:");
    process(test_input);

    let input = fs::read_to_string("data/day5.txt").expect("Failed to read the file.");
    println!("Real Input:");
    process(&input);
}
