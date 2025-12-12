use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use advent_of_code_2025::time;
use anyhow::Result;
use const_format::concatcp;

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("../input/", DAY, ".txt");

const TEST: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

const TEST_SOLUTION_PART1: usize = 50;
const TEST_SOLUTION_PART2: usize = 24;

fn part1(reader: &mut dyn BufRead) -> Result<usize> {
    let tiles = reader
        .lines()
        .flatten()
        .map(|l| {
            let (a, b) = l.split_once(',').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect::<Vec<(usize, usize)>>();

    let mut max = 0;

    for (i, &(x1, y1)) in tiles.iter().enumerate() {
        for &(x2, y2) in &tiles[i + 1..] {
            let area = (x1.max(x2) - x1.min(x2) + 1) * (y1.max(y2) - y1.min(y2) + 1);

            max = max.max(area);
        }
    }

    Ok(max)
}

#[derive(Debug, Copy, Clone)]
struct Edge {
    x1: usize,
    x2: usize,
    y1: usize,
    y2: usize,
}

impl Edge {
    fn new(x1: usize, x2: usize, y1: usize, y2: usize) -> Self {
        Self { x1, x2, y1, y2 }
    }
}

fn part2(reader: &mut dyn BufRead) -> Result<usize> {
    let mut tiles = vec![];
    let mut edges = vec![];

    for line in reader.lines().flatten() {
        let (l, r) = line.split_once(',').unwrap();
        let (x, y) = (l.parse()?, r.parse()?);
        if let Some(&(last_x, last_y)) = tiles.last() {
            edges.push(Edge::new(x, last_x, y, last_y));
        }
        tiles.push((x, y));
    }

    edges.push(Edge::new(
        tiles[0].0,
        tiles.last().unwrap().0,
        tiles[0].1,
        tiles.last().unwrap().1,
    ));

    let mut result = 0;

    for (i, &(x1, y1)) in tiles.iter().enumerate() {
        'inner: for &(x2, y2) in tiles[i + 1..].iter() {
            let min_x = x1.min(x2);
            let max_x = x1.max(x2);
            let min_y = y1.min(y2);
            let max_y = y1.max(y2);
            for &edge in &edges {
                let e_min_x = edge.x1.min(edge.x2);
                let e_max_x = edge.x1.max(edge.x2);
                let e_min_y = edge.y1.min(edge.y2);
                let e_max_y = edge.y1.max(edge.y2);

                if min_x < e_max_x && max_x > e_min_x && min_y < e_max_y && max_y > e_min_y {
                    continue 'inner;
                }
            }

            let area = (max_x - min_x + 1) * (max_y - min_y + 1);
            result = result.max(area);
        }
    }

    Ok(result)
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
