use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use advent_of_code_2025::time;
use anyhow::Result;
use const_format::concatcp;
use rayon::prelude::*;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("../input/", DAY, ".txt");

const TEST: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124
";

const TEST_SOLUTION_PART1: usize = 1_227_775_554;
const TEST_SOLUTION_PART2: usize = 4_174_379_265;

fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
    let mut line = String::new();
    reader.read_to_string(&mut line)?;

    let ranges = line.trim().split(',').collect::<Vec<_>>();

    let result = ranges
        .into_par_iter()
        .map(|range| {
            let mut result = 0;
            let (from, to) = range.split_once('-').expect("range has '-'");
            let range = from.parse::<usize>().unwrap()..=to.parse::<usize>().unwrap();

            for i in range {
                let mag = i.ilog10() + 1;

                if mag % 2 == 0 {
                    let mut cmp = i % 10usize.pow(mag / 2);
                    cmp = cmp * 10usize.pow(mag / 2) + cmp;
                    if cmp == i {
                        result += i;
                    }
                }
            }
            result
        })
        .sum();

    Ok(result)
}

fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
    let mut line = String::new();
    reader.read_to_string(&mut line)?;

    let ranges = line.trim().split(',').collect::<Vec<_>>();

    let result = ranges
        .into_par_iter()
        .map(|range| {
            let (from, to) = range.split_once('-').expect("range has '-'");
            let range = from.parse::<usize>().unwrap()..=to.parse::<usize>().unwrap();
            let mut result = 0;
            for i in range {
                let mag = i.ilog10() + 1;
                let mut is_invalid = false;

                for l in 1..=(mag / 2) {
                    if mag % l != 0 {
                        continue;
                    }

                    let ten_to_l = 10usize.pow(l);

                    let prefix = i % ten_to_l;
                    if prefix == 0 {
                        continue;
                    }

                    let mut compare = prefix;
                    while compare < i {
                        compare = compare * ten_to_l + prefix;
                    }

                    if compare == i {
                        is_invalid = true;
                        break;
                    }
                }

                if is_invalid {
                    result += i;
                }
            }
            result
        })
        .sum();

    Ok(result)
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
