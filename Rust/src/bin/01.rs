use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use advent_of_code_2025::time;
use anyhow::Result;
use const_format::concatcp;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("../input/", DAY, ".txt");

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

const TEST_SOLUTION_PART1: usize = 3;
const TEST_SOLUTION_PART2: usize = 6;

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let mut position = 50;
    let mut zeros = 0;

    for line in reader.lines() {
        let line = line?;
        let (dir, amount) = line.split_at(1);
        let amount = amount.parse::<i32>()?;
        match dir {
            "L" => position -= amount,
            "R" => position += amount,
            _ => panic!("must be L or R"),
        }

        position %= 100;

        if position == 0 {
            zeros += 1;
        }
    }

    Ok(zeros)
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let mut position = 50;
    let mut zeros = 0;
    let mut reflected = false;

    for line in reader.lines() {
        let line = line?;
        let (dir, amount) = line.split_at(1);
        let amount = amount.parse::<usize>()?;

        match dir {
            "L" => {
                if !reflected {
                    position = (100 - position) % 100;
                    reflected = true;
                }
                position += amount;
            }
            "R" => {
                if reflected {
                    position = (100 - position) % 100;
                    reflected = false;
                }
                position += amount
            }
            _ => panic!("must be L or R"),
        }

        zeros += position / 100;
        position %= 100;
    }

    Ok(zeros)
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
