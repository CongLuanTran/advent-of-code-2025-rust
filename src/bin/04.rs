#![allow(unexpected_cfgs)]

use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

const DIRECTION: [(isize, isize); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let grid: Vec<Vec<char>> = reader
            .lines()
            .map_while(|line| {
                if let Result::Ok(line) = line {
                    let chars: Vec<char> = line.chars().collect();
                    return Some(chars);
                }
                None
            })
            .collect();
        let h = grid.len();
        let w = grid[0].len();
        let mut cnt = 0;

        for y in 0..h {
            for x in 0..w {
                if grid[y][x] != '@' {
                    continue;
                }
                let mut adj = 0;
                for (dx, dy) in DIRECTION {
                    let ny = y as isize + dy;
                    let nx = x as isize + dx;
                    if ny < 0 || ny as usize >= h || nx < 0 || nx as usize >= w {
                        continue;
                    }
                    if grid[ny as usize][nx as usize] == '@' {
                        adj += 1;
                    }
                    if adj >= 4 {
                        break;
                    }
                }
                if adj < 4 {
                    cnt += 1;
                }
            }
        }

        Ok(cnt)
    }

    assert_eq!(13, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut grid: Vec<Vec<char>> = reader
            .lines()
            .map_while(|line| {
                if let Result::Ok(line) = line {
                    let chars: Vec<char> = line.chars().collect();
                    return Some(chars);
                }
                None
            })
            .collect();
        let h = grid.len();
        let w = grid[0].len();
        let mut cnt = 0;

        loop {
            let mut can_remove = 0;
            for y in 0..h {
                for x in 0..w {
                    if grid[y][x] != '@' {
                        continue;
                    }
                    let mut adj = 0;
                    for (dx, dy) in DIRECTION {
                        let ny = y as isize + dy;
                        let nx = x as isize + dx;
                        if ny < 0 || ny as usize >= h || nx < 0 || nx as usize >= w {
                            continue;
                        }
                        if grid[ny as usize][nx as usize] == '@' {
                            adj += 1;
                        }
                        if adj >= 4 {
                            break;
                        }
                    }
                    if adj < 4 {
                        can_remove += 1;
                        grid[y][x] = 'x';
                    }
                }
            }

            if can_remove == 0 {
                break;
            }
            cnt += can_remove;
        }
        Ok(cnt)
    }

    assert_eq!(43, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
