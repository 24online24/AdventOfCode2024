use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use advent_of_code2024::start_day;
use anyhow::Result;
use code_timing_macros::time_snippet;
use const_format::concatcp;
const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
const TEST2: &str = "\
89 91 92 95 93 94
";

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut safe_count = 0;
        for line in reader.lines() {
            let levels = line
                .unwrap()
                .split(' ')
                .map(|level_str| level_str.parse().unwrap())
                .collect::<Vec<i32>>();
            let mut dif = levels[1] - levels[0];
            if !(1..=3).contains(&dif.abs()) {
                continue;
            }
            let increasing = dif > 0;
            let mut i = 2;
            while i < levels.len() {
                dif = levels[i] - levels[i - 1];
                if !((1..=3).contains(&dif.abs()) && (dif > 0) == increasing) {
                    break;
                }
                i += 1;
            }
            if i == levels.len() {
                safe_count += 1;
            }
        }
        Ok(safe_count)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut safe_count = 0;
        for line in reader.lines() {
            let levels = line
                .unwrap()
                .split(' ')
                .map(|level_str| level_str.parse().unwrap())
                .collect::<Vec<i32>>();
            let mut dif = levels[1] - levels[0];
            let mut last_bad = false;
            let mut found_bad = false;
            if !(1..=3).contains(&dif.abs()) {
                last_bad = true;
                found_bad = true;
            }
            let increasing = dif > 0;
            let mut i = 2;
            while i < levels.len() {
                if last_bad {
                    dif = levels[i] - levels[i - 2];
                } else {
                    dif = levels[i] - levels[i - 1];
                }
                last_bad = false;
                if !((1..=3).contains(&dif.abs()) && (dif > 0) == increasing) {
                    if found_bad {
                        break;
                    } else {
                        last_bad = true;
                        found_bad = true;
                        i += 1;
                        continue;
                    }
                }
                i += 1;
            }
            if i == levels.len() {
                safe_count += 1;
            }
        }
        Ok(safe_count)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);
    assert_eq!(1, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
