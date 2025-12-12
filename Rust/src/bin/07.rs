use advent_of_code_2025::time;
use anyhow::Result;
use const_format::concatcp;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("../input/", DAY, ".txt");

const TEST: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

const TEST_SOLUTION_PART1: usize = 21;
const TEST_SOLUTION_PART2: usize = 40;

fn part1(reader: &mut dyn BufRead) -> Result<usize> {
    let mut lines = reader.lines().flatten();
    let start = lines.next().unwrap();
    let start_i = start.bytes().position(|b| b == b'S').unwrap();
    let mut beams = vec![false; start.len()];
    beams[start_i] = true;

    let mut splits = 0;

    for line in lines.skip(1).step_by(2) {
        for (i, b) in line.bytes().enumerate() {
            if b == b'^' && beams[i] {
                splits += 1;
                beams[i] = false;
                beams[i + 1] = true;
                beams[i - 1] = true;
            }
        }
    }

    Ok(splits)
}

fn part2(reader: &mut dyn BufRead) -> Result<usize> {
    let mut lines = reader.lines().flatten();
    let start = lines.next().unwrap();
    let start_i = start.bytes().position(|b| b == b'S').unwrap();
    let mut beams = vec![0; start.len()];
    beams[start_i] = 1;

    for line in lines.skip(1).step_by(2) {
        for (i, b) in line.bytes().enumerate() {
            if b == b'^' {
                beams[i + 1] += beams[i];
                beams[i - 1] += beams[i];
                beams[i] = 0;
            }
        }
    }

    Ok(beams.into_iter().sum())
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
