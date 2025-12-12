use advent_of_code_2025::time;
use anyhow::Result;
use const_format::concatcp;
use std::collections::{HashMap, VecDeque};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("../input/", DAY, ".txt");

const TESTS: [&str; 2] = [
    "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
",
    "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
",
];

const TEST_SOLUTION_PART1: usize = 5;
const TEST_SOLUTION_PART2: usize = 2;

fn part1(reader: &mut dyn BufRead) -> Result<usize> {
    let map = reader
        .lines()
        .flatten()
        .map(|line| {
            let parts = line
                .split_whitespace()
                .map(str::to_string)
                .collect::<Vec<_>>();
            (parts[0][..3].to_string(), parts[1..].to_vec())
        })
        .collect::<HashMap<_, _>>();

    let mut queue = VecDeque::from_iter(&map["you"]);

    let mut count = 0;
    while let Some(next) = queue.pop_front() {
        if next == "out" {
            count += 1;
        } else {
            queue.extend(&map[next]);
        }
    }

    Ok(count)
}

fn part2(reader: &mut dyn BufRead) -> Result<usize> {
    let map = reader
        .lines()
        .flatten()
        .map(|line| {
            let parts = line
                .split_whitespace()
                .map(str::to_string)
                .collect::<Vec<_>>();
            (parts[0][..3].to_string(), parts[1..].to_vec())
        })
        .collect::<HashMap<_, _>>();

    Ok(count_paths("svr", false, false, &map, &mut HashMap::new()))
}

fn count_paths<'a>(
    device: &'a str,
    dac: bool,
    fft: bool,
    map: &'a HashMap<String, Vec<String>>,
    cache: &mut HashMap<(&'a str, bool, bool), usize>,
) -> usize {
    if let Some(&n) = cache.get(&(device, dac, fft)) {
        return n;
    }

    if device == "out" {
        return usize::from(dac && fft);
    }

    let n_dac = dac || device == "dac";
    let n_fft = fft || device == "fft";

    let n = map[device]
        .iter()
        .map(|dev| count_paths(dev, n_dac, n_fft, map, cache))
        .sum();

    cache.insert((device, dac, fft), n);

    n
}

//region main

fn main() -> Result<()> {
    println!(" === Day: {DAY} === ");

    run_part(1, TEST_SOLUTION_PART1, part1)?;
    run_part(2, TEST_SOLUTION_PART2, part2)
}

fn run_part<F>(number: usize, expected: usize, part: F) -> Result<()>
where
    F: Fn(&mut dyn BufRead) -> Result<usize>,
{
    println!("\n  == Part {number:02} ==");

    let mut test = BufReader::new(TESTS[number - 1].as_bytes());
    assert_eq!(expected, part(&mut test)?);

    let mut input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time!(part(&mut input_file)?);
    println!("Result: {result}");

    Ok(())
}

//endregion
