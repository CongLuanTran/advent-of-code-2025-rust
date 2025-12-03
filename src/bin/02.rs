#![allow(unexpected_cfgs)]

use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124\
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn to_range(s: &str) -> (&str, &str) {
        let (a, b) = s.split_once("-").unwrap();
        (a, b)
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let line = reader.lines().map_while(Result::ok).next().unwrap();
        let ranges = line.split(",").map(to_range);
        let mut invalids: HashSet<usize> = HashSet::new();
        for (lo, hi) in ranges {
            let lo_val = lo.parse().unwrap();
            let hi_val = hi.parse().unwrap();
            for k in lo.len().div_ceil(2)..=hi.len() / 2 {
                let k = k as u32;
                for i in 10usize.pow(k - 1)..10usize.pow(k) {
                    let full = i.to_string().repeat(2).parse().unwrap();
                    if full >= lo_val && full <= hi_val {
                        invalids.insert(full);
                    }
                }
            }
        }
        Ok(invalids.iter().sum())
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(1227775554, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let line = reader.lines().map_while(Result::ok).next().unwrap();
        let ranges = line.split(",").map(to_range);
        let mut invalids: HashSet<usize> = HashSet::new();
        for (lo, hi) in ranges {
            let lo_val = lo.parse().unwrap();
            let hi_val = hi.parse().unwrap();
            for n in lo.len()..=hi.len() {
                for k in 1..=(n / 2) {
                    if n % k != 0 {
                        continue;
                    }
                    let m = n / k;
                    let k = k as u32;
                    for i in 10usize.pow(k - 1)..10usize.pow(k) {
                        let full = i.to_string().repeat(m).parse().unwrap();
                        if full >= lo_val && full <= hi_val {
                            invalids.insert(full);
                        }
                    }
                }
            }
        }
        Ok(invalids.iter().sum())
    }

    assert_eq!(4174379265, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
