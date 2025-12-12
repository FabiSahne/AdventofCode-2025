use advent_of_code_2025::time;
use anyhow::Result;
use const_format::concatcp;
use std::str::FromStr;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("../input/", DAY, ".txt");

//region main

fn main() -> Result<()> {
    println!(" === Day: {DAY} === ");

    let reader = BufReader::new(File::open(INPUT_FILE)?);
    println!("Result: {}", time!(run_part(reader)?));
    Ok(())
}

fn run_part<R: BufRead>(reader: R) -> Result<usize> {
    let lines = reader.lines().flatten().collect::<Vec<_>>();

    let result = lines[30..]
        .into_iter()
        .filter(|tree| {
            let number = tree
                .split([':', ' ', 'x'])
                .filter(|s| !s.is_empty())
                .flat_map(i32::from_str)
                .collect::<Vec<_>>();
            let width = number[0];
            let height = number[1];
            let required = number[2..].iter().sum();

            width / 3 * height / 3 >= required
        })
        .count();

    Ok(result)
}

//endregion
