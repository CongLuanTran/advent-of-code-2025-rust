#![allow(unexpected_cfgs)]

use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse_line(line: &str) -> Result<i32> {
        let (sign, value) = line.split_at(1);
        let value = value.parse::<i32>()?;
        Ok(if sign == "R" { value } else { -value })
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .map(|line| parse_line(&line?))
            .try_fold((0usize, 50), |(cnt, val), n| {
                let n = n?;
                let cnt = if (val + n) % 100 == 0 { cnt + 1 } else { cnt };
                Ok((cnt, val + n))
            })?
            .0;
        Ok(answer)
    }

    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .map(|line| parse_line(&line?))
            .try_fold((0usize, 50), |(cnt, val), n| {
                let n = n?;
                let (q, r) = (n / 100, n % 100);
                let mut cnt = cnt + q.unsigned_abs() as usize;

                if r != 0 && val != 0 && (val + r >= 100 || val + r <= 0) {
                    cnt += 1;
                }

                Ok((cnt, (val + r).rem_euclid(100)))
            })?
            .0;
        Ok(answer)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
