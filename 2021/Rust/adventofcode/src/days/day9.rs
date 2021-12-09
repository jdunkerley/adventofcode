use crate::shared::utils::lines;
use std::fs;

fn process(input: &str) {
    let lines = lines(input);

    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(lines.len());
    for line in lines {
        let row: Vec<u8> = line.chars().map(|c| (c as u8 - '0' as u8)).collect();
        grid.push(row);
    }

    let mut total = 0_i32;
    let mut basins = [0, 0, 0];
    let height = grid.len();
    let width = grid[0].len();
    for r in 0..height {
        for c in 0..width {
            if (r == 0 || grid[r][c] < grid[r - 1][c])
                && (c == 0 || grid[r][c] < grid[r][c - 1])
                && (r + 1 == height || grid[r][c] < grid[r + 1][c])
                && (c + 1 == width || grid[r][c] < grid[r][c + 1])
            {
                //println!("{} {} {}", r, c, grid[r][c]);
                total += 1 + grid[r][c] as i32;
                let basin = find_basin(&grid, r, c, width, height);
                if basin > basins[0] {
                    basins[2] = basins[1];
                    basins[1] = basins[0];
                    basins[0] = basin;
                } else if basin > basins[1] {
                    basins[2] = basins[1];
                    basins[1] = basin;
                } else if basin > basins[2] {
                    basins[2] = basin;
                }
            }
        }
    }

    println!("{} {}", total, basins[0] * basins[1] * basins[2]);
}

fn find_basin(grid: &Vec<Vec<u8>>, r: usize, c: usize, width: usize, height: usize) -> usize {
    let mut basin = vec![(r, c)];

    let mut added = true;
    while added {
        added = false;

        let len = basin.len();
        for i in 0..len {
            let start = basin[i];

            if start.0 > 0
                && grid[start.0 - 1][start.1] != 9
                && !basin.contains(&(start.0 - 1, start.1))
            {
                basin.push((start.0 - 1, start.1));
                added = true;
            }
            if start.1 > 0
                && grid[start.0][start.1 - 1] != 9
                && !basin.contains(&(start.0, start.1 - 1))
            {
                basin.push((start.0, start.1 - 1));
                added = true;
            }
            if start.0 + 1 < height
                && grid[start.0 + 1][start.1] != 9
                && !basin.contains(&(start.0 + 1, start.1))
            {
                basin.push((start.0 + 1, start.1));
                added = true;
            }
            if start.1 + 1 < width
                && grid[start.0][start.1 + 1] != 9
                && !basin.contains(&(start.0, start.1 + 1))
            {
                basin.push((start.0, start.1 + 1));
                added = true;
            }
        }
    }

    basin.len()
}

pub fn run() {
    let test_input = "
2199943210
3987894921
9856789892
8767896789
9899965678";
    println!("Test Input:");
    process(test_input);

    let input = fs::read_to_string("data/day9.txt").expect("Failed to read the file.");
    println!("Real Input:");
    process(&input);
}
