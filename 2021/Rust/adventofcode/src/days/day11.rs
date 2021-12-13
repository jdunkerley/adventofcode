use crate::shared::utils::lines;
use std::fs;

fn offset_index(start: (usize, usize), offset: (i8, i8)) -> Option<(usize, usize)> {
    let temp = (start.0 as i8 + offset.0, start.1 as i8 + offset.1);
    if temp.0 < 0 || temp.0 > 9 || temp.1 < 0 || temp.1 > 9 {
        None
    } else {
        Some((temp.0 as usize, temp.1 as usize))
    }
}

fn print_grid(grid: &[[u8; 10]; 10]) {
    for row in grid {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
    println!();
}

fn simulate(lines: &Vec<String>) {
    let mut grid = [[0u8; 10]; 10];
    for i in 0..10 {
        let line: Vec<u8> = lines[i]
            .chars()
            .map(|c| (c as i8 - '0' as i8) as u8)
            .collect();
        for j in 0..10 {
            grid[i][j] = line[j];
        }
    }

    print_grid(&grid);

    let offsets = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let mut flashes = 0u32;
    let mut iter = 0;
    loop {
        let mut nines = Vec::new();

        // Add 1 to all cells
        for i in 0..10 {
            for j in 0..10 {
                grid[i][j] += 1;
                if grid[i][j] == 10 {
                    nines.push((i, j));
                }
            }
        }

        // Process the 9s
        let cflashes = flashes;
        let mut current = nines.pop();
        while let Some((i, j)) = current {
            grid[i][j] = 0;
            flashes += 1;
            for offset in offsets {
                if let Some((ii, jj)) = offset_index((i, j), offset) {
                    if grid[ii][jj] != 0 {
                        grid[ii][jj] += 1;
                        if grid[ii][jj] == 10 {
                            nines.push((ii, jj));
                        }
                    }
                }
            }
            current = nines.pop();
        }

        iter += 1;
        if flashes - cflashes == 100 {
            println!("All 0 at {}", iter);
            break;
        }
        if iter == 100 {
            println!("Flashes @ 100 = {}", flashes);
        }
    }
}

fn process(input: &str) {
    let lines = lines(input);
    simulate(&lines);
}

pub fn run() {
    let test_input = "
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
    println!("Test Input:");
    process(test_input);

    let input = fs::read_to_string("data/day11.txt").expect("Failed to read the file.");
    println!("Real Input:");
    process(&input);
}
