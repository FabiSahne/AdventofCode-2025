use advent_of_code_2025::time;
use anyhow::Result;
use const_format::concatcp;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

const TEST_SOLUTION_PART1: usize = 40;
const TEST_SOLUTION_PART2: usize = 25272;

static IS_TEST: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Ord, Eq)]
struct Vec3(usize, usize, usize);

impl Vec3 {
    fn distance_squared(self, other: Self) -> usize {
        self.0.abs_diff(other.0) * self.0.abs_diff(other.0)
            + self.1.abs_diff(other.1) * self.1.abs_diff(other.1)
            + self.2.abs_diff(other.2) * self.2.abs_diff(other.2)
    }
}

#[derive(Debug, Copy, Clone)]
struct Pair(Vec3, Vec3);

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.0.distance_squared(self.1) == other.0.distance_squared(other.1)
    }
}

impl Eq for Pair {}
impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .0
            .distance_squared(other.1)
            .cmp(&self.0.distance_squared(self.1))
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

fn part1(reader: &mut dyn BufRead) -> Result<usize> {
    let number_to_connect = if IS_TEST.load(Ordering::Relaxed) {
        10
    } else {
        1000
    };

    let boxes = reader
        .lines()
        .flatten()
        .map(|line| {
            let mut parts = line.split(',').map(usize::from_str).flatten();
            Vec3(
                parts.next().unwrap(),
                parts.next().unwrap(),
                parts.next().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut pairs = BinaryHeap::new();
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            pairs.push(Pair(boxes[i], boxes[j]));
        }
    }

    let mut circuits = boxes
        .into_iter()
        .map(|b| HashSet::from_iter(std::iter::once(b)))
        .collect::<Vec<HashSet<Vec3>>>();

    for _ in 0..number_to_connect {
        let Pair(box1, box2) = pairs.pop().unwrap();

        let c1_idx = circuits.iter().position(|c| c.contains(&box1)).unwrap();
        let c2_idx = circuits.iter().position(|c| c.contains(&box2)).unwrap();

        if c1_idx == c2_idx {
            continue;
        }

        let c2 = circuits[c2_idx].clone();
        circuits[c1_idx].extend(c2);
        circuits.remove(c2_idx);
    }

    let mut lengths = circuits.into_iter().map(|c| c.len()).collect::<Vec<_>>();
    lengths.sort_unstable_by_key(|&l| Reverse(l));

    Ok(lengths
        .into_iter()
        .take(3)
        .reduce(|prod, len| prod * len)
        .unwrap())
}

fn part2(reader: &mut dyn BufRead) -> Result<usize> {
    let boxes = reader
        .lines()
        .flatten()
        .map(|line| {
            let mut parts = line.split(',').map(usize::from_str).flatten();
            Vec3(
                parts.next().unwrap(),
                parts.next().unwrap(),
                parts.next().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut pairs = BinaryHeap::new();
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            pairs.push(Pair(boxes[i], boxes[j]));
        }
    }

    let mut circuits = boxes
        .into_iter()
        .map(|b| HashSet::from_iter(std::iter::once(b)))
        .collect::<Vec<HashSet<Vec3>>>();

    while circuits.len() > 1 {
        let Pair(box1, box2) = pairs.pop().unwrap();

        let c1_idx = circuits.iter().position(|c| c.contains(&box1)).unwrap();
        let c2_idx = circuits.iter().position(|c| c.contains(&box2)).unwrap();

        if c1_idx == c2_idx {
            continue;
        }

        let c2 = circuits[c2_idx].clone();
        circuits[c1_idx].extend(c2);
        circuits.remove(c2_idx);

        if circuits.len() == 1 {
            return Ok(box1.0 * box2.0);
        }
    }

    Ok(0)
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

    IS_TEST.swap(true, Ordering::Release);
    let mut test = BufReader::new(TEST.as_bytes());
    assert_eq!(expected, part(&mut test)?);

    IS_TEST.swap(false, Ordering::Release);
    let mut input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time!(part(&mut input_file)?);
    println!("Result: {result}");

    Ok(())
}

//endregion
