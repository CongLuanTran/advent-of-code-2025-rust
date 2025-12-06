#![allow(unexpected_cfgs)]

use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn joltagens(bats: String, len: usize) -> usize {
        let bats: Vec<char> = bats.trim().chars().collect();
        let l_bats = bats.len();
        let mut joltagens = String::new();
        let mut s = 0;
        for i in (1..=len).rev() {
            let valid_ranges = &bats[s..=(l_bats - i)];
            if let Some(max) = valid_ranges.iter().max() {
                if let Some(p) = valid_ranges.iter().position(|v| v == max) {
                    joltagens.push(valid_ranges[p]);
                    s += p + 1;
                }
            }
        }

        joltagens.parse().unwrap()
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .map_while(|line| {
                if let Result::Ok(bats) = line {
                    let joltagens = joltagens(bats, 2);
                    Some(joltagens)
                } else {
                    None
                }
            })
            .sum();
        Ok(answer)
    }

    assert_eq!(357, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .map_while(|line| {
                if let Result::Ok(bats) = line {
                    let joltagens = joltagens(bats, 12);
                    Some(joltagens)
                } else {
                    None
                }
            })
            .sum();
        Ok(answer)
    }

    assert_eq!(3121910778619, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
