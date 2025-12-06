#![allow(unexpected_cfgs)]

use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut it = reader.lines();
        let mut ranges: Vec<(usize, usize)> = it
            .by_ref()
            .map_while(|line| {
                line.ok()
                    .and_then(|l| if l.is_empty() { None } else { Some(l) })
            })
            .map(|line| {
                let (a, b) = line.split_once("-").unwrap();
                (a.parse().unwrap(), b.parse().unwrap())
            })
            .collect();
        let mut items: Vec<usize> = it
            .map_while(Result::ok)
            .map(|l| l.parse().unwrap())
            .collect();
        ranges.sort();
        items.sort();

        let answer = items
            .iter()
            .filter(|&i| ranges.iter().any(|(a, b)| a <= i && i <= b))
            .count();
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
        let mut it = reader.lines();
        let mut ranges: Vec<(usize, usize)> = it
            .by_ref()
            .map_while(|line| {
                line.ok()
                    .and_then(|l| if l.is_empty() { None } else { Some(l) })
            })
            .map(|line| {
                let (a, b) = line.split_once("-").unwrap();
                (a.parse().unwrap(), b.parse().unwrap())
            })
            .collect();
        ranges.sort();

        let answer = ranges
            .iter()
            .fold((0, 0), |(sum, base), &(a, b)| match base {
                base if base < a => (sum + b - a + 1, b),
                base if base < b => (sum + b - base, b),
                _ => (sum, base),
            })
            .0;

        Ok(answer)
    }

    assert_eq!(14, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
