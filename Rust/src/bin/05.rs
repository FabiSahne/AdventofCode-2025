use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use advent_of_code_2025::time;
use anyhow::Result;
use const_format::concatcp;

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("../input/", DAY, ".txt");

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

const TEST_SOLUTION_PART1: usize = 3;
const TEST_SOLUTION_PART2: usize = 14;

#[derive(Debug, Copy, Clone)]
struct Range(usize, usize);

impl Range {
    #[inline]
    fn begin(&self) -> usize {
        self.0
    }

    #[inline]
    fn end(&self) -> usize {
        self.1
    }

    #[inline]
    fn size(&self) -> usize {
        self.1 - self.0 + 1
    }

    #[inline]
    fn contains(&self, val: usize) -> bool {
        val >= self.begin() && val <= self.end()
    }
}

fn part1(reader: &mut dyn BufRead) -> Result<usize> {
    let mut ranges = vec![];
    let mut ingredients = vec![];

    let mut read_ranges = false;

    for line in reader.lines().flatten() {
        if line.is_empty() {
            read_ranges = true;
            continue;
        }

        if read_ranges {
            ingredients.push(line.parse::<usize>()?);
        } else {
            let (from, to) = line.split_once('-').unwrap();
            ranges.push(Range(from.parse()?, to.parse()?));
        }
    }

    Ok(ingredients
        .into_iter()
        .filter(|&i| ranges.iter().any(|r| r.contains(i)))
        .count())
}

fn part2(reader: &mut dyn BufRead) -> Result<usize> {
    let mut ranges = vec![];

    for line in reader.lines().flatten() {
        if line.is_empty() {
            break;
        }
        let (from, to) = line.split_once('-').unwrap();
        ranges.push(Range(from.parse()?, to.parse()?));
    }

    ranges.sort_unstable_by_key(|r| r.1);

    for i in (1..ranges.len()).rev() {
        if ranges[i - 1].end() >= ranges[i].begin() {
            ranges[i - 1] = Range(
                ranges[i - 1].begin().min(ranges[i].begin()),
                ranges[i].end(),
            );
            ranges.remove(i);
        }
    }

    Ok(ranges
        .into_iter()
        .fold(0, |total, range| total + range.size()))
}

fn main() -> Result<()> {
    println!(" === Day: {DAY} === ");

    run_part(1, TEST_SOLUTION_PART1, part1)?;
    run_part(2, TEST_SOLUTION_PART2, part2)
}

fn run_part<F>(number: u8, expected: usize, part: F) -> Result<()>
where
    F: Fn(&mut dyn BufRead) -> Result<usize>,
{
    println!("\n  == Part {number:02} ==");

    let mut test = BufReader::new(TEST.as_bytes());
    assert_eq!(expected, part(&mut test)?);

    let mut input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time!(part(&mut input_file)?);
    println!("Result: {result}");

    Ok(())
}
