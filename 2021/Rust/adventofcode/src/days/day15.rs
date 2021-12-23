use crate::shared::utils::lines;
use std::fs;
use std::iter::repeat;

fn repeat_grid(input: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut grid: Vec<Vec<i32>> = Vec::with_capacity(input.len() * 5);

    for y in 0..5 {
        for row in input {
            let mut new_row = Vec::with_capacity(row.len());
            for x in 0..5 {
                for val in row {
                    let mut new_val = val + x + y;
                    if new_val > 9 {
                        new_val -= 9;
                    }
                    new_row.push(new_val);
                }
            }
            grid.push(new_row);
        }
    }

    grid
}

fn process(input: &str) {
    let grid: Vec<Vec<i32>> = lines(input)
        .iter()
        .map(|s| {
            s.chars()
                .map(|x| (x as i32 - '0' as i32))
                .collect::<Vec<i32>>()
        })
        .collect();

    let grid = repeat_grid(&grid);

    let height = grid.len();
    let width = grid[0].len();
    let mut cost: Vec<Vec<i32>> = Vec::with_capacity(height);
    for _ in 0..height {
        cost.push(repeat(0).take(width).collect());
    }

    let mut positions = vec![(0, 0)];
    while !positions.is_empty() {
        let mut new_positions = Vec::new();

        for position in positions {
            for shift in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                let new = (position.0 as i64 + shift.0, position.1 as i64 + shift.1);
                if new.0 < 0
                    || new.1 < 0
                    || new.0 >= height as i64
                    || new.1 >= width as i64
                    || (new.0 == 0 && new.1 == 0)
                {
                    continue;
                }

                let new = (new.0 as usize, new.1 as usize);

                let new_cost = cost[position.0][position.1] + grid[new.0][new.1];
                if cost[new.0][new.1] == 0 || cost[new.0][new.1] > new_cost {
                    cost[new.0][new.1] = new_cost;
                    new_positions.push(new);
                }
            }
        }

        positions = new_positions;
    }

    println!("{}", cost[height - 1][width - 1]);
}

pub fn run() {
    let test_input = "
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
    println!("Test Input:");
    process(test_input);

    let input = fs::read_to_string("data/day15.txt").expect("Failed to read the file.");
    println!("Real Input:");
    process(&input);
}
