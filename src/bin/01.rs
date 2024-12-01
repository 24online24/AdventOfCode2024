use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use advent_of_code2024::start_day;
use anyhow::Result;
use code_timing_macros::time_snippet;
use const_format::concatcp;
const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut list1 = Vec::new();
        let mut list2 = Vec::new();
        reader.lines().for_each(|line| {
            let l = line.unwrap();
            let values = l.split_whitespace().collect::<Vec<&str>>();
            list1.push(values[0].parse::<i32>().unwrap());
            list2.push(values[1].parse::<i32>().unwrap());
        });
        list1.sort();
        list2.sort();
        let mut dif = 0;
        for i in 0..list1.len() {
            dif += (list1[i] - list2[i]).unsigned_abs();
        }
        Ok(dif as usize)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut list1 = Vec::new();
        let mut list2 = Vec::new();
        reader.lines().for_each(|line| {
            let l = line.unwrap();
            let values = l.split_whitespace().collect::<Vec<&str>>();
            list1.push(values[0].parse::<usize>().unwrap());
            list2.push(values[1].parse::<usize>().unwrap());
        });
        list1.sort();
        list2.sort();
        let mut score = 0;
        let mut i = 0;
        let mut j = 0;
        while i < list1.len() {
            let number = list1[i];
            let mut occurances = 0;
            while j < list2.len() && list2[j] < number {
                j += 1;
            }
            while j < list2.len() && list2[j] == number {
                occurances += 1;
                j += 1;
            }
            while i < list1.len() && list1[i] == number {
                score += number * occurances;
                i += 1;
            }
        }

        Ok(score)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
