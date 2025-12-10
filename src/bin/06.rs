use advent_of_code_2025::time;
use anyhow::Result;
use const_format::concatcp;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

const TEST_SOLUTION_PART1: usize = 4277556;
const TEST_SOLUTION_PART2: usize = 3263827;

fn part1(reader: &mut dyn BufRead) -> Result<usize> {
    let mut problems = vec![];
    let mut ops = vec![];

    for line in reader.lines().flatten() {
        for (i, part) in line.split_whitespace().enumerate() {
            if let Ok(num) = part.parse::<usize>() {
                if problems.get(i).is_none() {
                    problems.push(vec![num]);
                } else {
                    problems[i].push(num);
                }
            } else {
                ops.push(part.parse::<Op>()?)
            }
        }
    }

    let result = problems
        .into_iter()
        .zip(ops.into_iter())
        .map(|(p, o)| o.calc(&p))
        .sum();

    Ok(result)
}

fn part2(reader: &mut dyn BufRead) -> Result<usize> {
    let mut lines = vec![];

    for line in reader.lines().flatten() {
        lines.push(line.into_bytes());
    }

    let width = lines[0].len();
    let height = lines.len();

    let mut nums = vec![];
    let mut ops = vec![];

    let mut i = width;
    let mut cur_nums = vec![];

    while i > 0 {
        i -= 1;

        let mut num = 0;
        for h in 0..(height - 1) {
            if lines[h][i].is_ascii_digit() {
                num *= 10;
                num += (lines[h][i] - b'0') as usize;
            }
        }
        cur_nums.push(num);

        if let Ok(op) = Op::try_from(lines[height - 1][i]) {
            ops.push(op);
            nums.push(cur_nums);
            cur_nums = vec![];
            i = i.saturating_sub(1);
        }
    }

    let result = nums
        .into_iter()
        .zip(ops.into_iter())
        .map(|(p, o)| o.calc(&p))
        .sum();

    Ok(result)
}

enum Op {
    Add,
    Mul,
}

impl Op {
    fn calc(&self, nums: &[usize]) -> usize {
        match self {
            Op::Add => nums.into_iter().sum(),
            Op::Mul => nums.into_iter().product(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct ParseOpError(u8);

impl Display for ParseOpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error Parsing Op: {:?}", self.0 as char)
    }
}
impl Error for ParseOpError {}

impl FromStr for Op {
    type Err = ParseOpError;

    fn from_str(s: &str) -> Result<Op, ParseOpError> {
        match s.as_bytes() {
            b"+" => Ok(Op::Add),
            b"*" => Ok(Op::Mul),
            [b, ..] => Err(ParseOpError(*b)),
            _ => Err(ParseOpError(0)),
        }
    }
}

impl TryFrom<u8> for Op {
    type Error = ParseOpError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            b'+' => Ok(Self::Add),
            b'*' => Ok(Self::Mul),
            _ => Err(ParseOpError(value)),
        }
    }
}

//region main

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

//endregion
