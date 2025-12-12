use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use advent_of_code_2025::time;
use anyhow::Result;
use const_format::concatcp;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("../input/", DAY, ".txt");

const TEST: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

const TEST_SOLUTION_PART1: usize = 13;
const TEST_SOLUTION_PART2: usize = 43;

const DIRS: [[usize; 2]; 8] = [
    [usize::MAX, 0],
    [usize::MAX, 1],
    [0, 1],
    [1, 1],
    [1, 0],
    [1, usize::MAX],
    [0, usize::MAX],
    [usize::MAX, usize::MAX],
];

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let mut accessible = 0;

    let mut grid = vec![];

    for line in reader.lines().flatten() {
        grid.push(line.into_bytes());
    }

    let (height, width) = (grid.len(), grid[0].len());

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] != b'@' {
                continue;
            }

            let mut count = 0;
            for dir in DIRS {
                let (dx, dy) = (x.wrapping_add(dir[0]), y.wrapping_add(dir[1]));
                if dx < width && dy < height && grid[dy][dx] == b'@' {
                    count += 1;
                }
            }

            accessible += usize::from(count < 4);
        }
    }

    Ok(accessible)
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let mut total = 0;

    let mut grid = vec![];
    for line in reader.lines().flatten() {
        grid.push(line.into_bytes());
    }

    let (height, width) = (grid.len(), grid[0].len());

    let mut removed = 1;
    while removed > 0 {
        removed = 0;

        for y in 0..height {
            for x in 0..width {
                if grid[y][x] != b'@' {
                    continue;
                }

                let mut count = 0;
                for dir in DIRS {
                    let (dx, dy) = (x.wrapping_add(dir[0]), y.wrapping_add(dir[1]));
                    if dx < width && dy < height && grid[dy][dx] == b'@' {
                        count += 1;
                    }
                }

                if count >= 4 {
                    continue;
                }

                grid[y][x] = b'x';
                removed += 1;
            }
        }

        total += removed;
    }

    Ok(total)
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
