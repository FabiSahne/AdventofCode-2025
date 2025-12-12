use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use advent_of_code_2025::time;
use anyhow::Result;
use const_format::concatcp;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("../input/", DAY, ".txt");

const TEST: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

const TEST_SOLUTION_PART1: usize = 357;
const TEST_SOLUTION_PART2: usize = 3_121_910_778_619;

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let mut sum = 0;

    for line in reader.lines().flatten() {
        let has_digit_one = &line[..line.len() - 1];
        let (idx, max1) = has_digit_one
            .bytes()
            .enumerate()
            .rev()
            .max_by_key(|&(_, c)| c)
            .unwrap();
        let has_digit_two = &line[idx + 1..];
        let max2 = has_digit_two.bytes().max().unwrap();
        sum += ((max1 - b'0') * 10 + max2 - b'0') as usize;
    }

    Ok(sum)
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let mut sum = 0;

    for line in reader.lines().flatten() {
        let mut batteries = 0;
        let mut left = 0;
        for right in (0..12).rev() {
            let has_digit = &line[left..line.len() - right];
            let (offset, max) = has_digit
                .bytes()
                .enumerate()
                .rev()
                .max_by_key(|&(_, c)| c)
                .unwrap();
            left += offset + 1;
            batteries = batteries * 10 + (max - b'0') as usize;
        }
        sum += batteries;
    }

    Ok(sum)
}

fn main() -> Result<()> {
    println!(" === Day: {DAY} === ");

    println!("  == Part 01 == ");

    assert_eq!(TEST_SOLUTION_PART1, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time!(part1(input_file)?);
    println!("Result: {result}");

    println!("\n  == Part 02 == ");

    assert_eq!(TEST_SOLUTION_PART2, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time!(part2(input_file)?);
    println!("Result: {result}");

    Ok(())
}
