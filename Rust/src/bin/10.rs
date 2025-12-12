use advent_of_code_2025::time;
use anyhow::Result;
use const_format::concatcp;
use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use z3::ast::Int;
use z3::{Optimize, SatResult};

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("../input/", DAY, ".txt");

const TEST: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

const TEST_SOLUTION_PART1: usize = 7;
const TEST_SOLUTION_PART2: usize = 33;

fn part1(reader: &mut dyn BufRead) -> Result<usize> {
    Ok(reader
        .lines()
        .flatten()
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            let lights = parts[0][1..parts[0].len() - 1]
                .bytes()
                .enumerate()
                .fold(0, |lights, (i, c)| lights ^ (u32::from(c == b'#') << i));
            let buttons = parts[1..parts.len() - 1]
                .iter()
                .map(|b| {
                    b[1..b.len() - 1]
                        .split(',')
                        .flat_map(u32::from_str)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            solve_via_bfs(lights, &buttons).unwrap()
        })
        .sum())
}

fn part2(reader: &mut dyn BufRead) -> Result<usize> {
    Ok(reader
        .lines()
        .flatten()
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            let joltage = parts
                .last()
                .unwrap()
                .trim_matches(['{', '}'])
                .split(',')
                .flat_map(u32::from_str)
                .collect::<Vec<_>>();
            let buttons = parts[1..parts.len() - 1]
                .iter()
                .map(|b| {
                    b[1..b.len() - 1]
                        .split(',')
                        .flat_map(u32::from_str)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            solve_via_z3(&joltage, &buttons).unwrap()
        })
        .sum())
}

fn solve_via_bfs(goal: u32, buttons: &[Vec<u32>]) -> Result<usize> {
    let mut queue = VecDeque::with_capacity(1 << 20);
    queue.push_back((0, 0));

    let mut seen = HashSet::new();
    seen.insert(0);

    while let Some((state, pressed)) = queue.pop_front() {
        if state == goal {
            return Ok(pressed);
        }

        seen.insert(state);

        let neighbors = buttons
            .iter()
            .map(|button| {
                button
                    .iter()
                    .fold(state, |current, wire| current ^ (1u32 << wire))
            })
            .filter(|neighbor| !seen.contains(neighbor));

        for neighbor in neighbors {
            queue.push_back((neighbor, pressed + 1));
        }
    }

    Err(anyhow::Error::msg("No valid button presses found"))
}

fn solve_via_z3(goal: &[u32], buttons: &[Vec<u32>]) -> Result<usize> {
    let opt = Optimize::new();

    let button_vars = (0..buttons.len())
        .map(|i| Int::new_const(i as u32))
        .collect::<Vec<_>>();

    let zero = Int::from_i64(0);
    for var in &button_vars {
        opt.assert(&var.ge(&zero));
    }

    for (counter_idx, &target) in goal.iter().enumerate() {
        let target = u64::from(target);

        let mut sum_terms = vec![];
        for (button_idx, button) in buttons.iter().enumerate() {
            if button.contains(&(counter_idx as u32)) {
                sum_terms.push(button_vars[button_idx].clone());
            }
        }

        if sum_terms.is_empty() {
            if target != 0 {
                return Err(anyhow::Error::msg("No valid button presses"));
            }
        } else {
            let sum = sum_terms.into_iter().reduce(|a, b| a + b).unwrap();
            let target_val = Int::from_u64(target);
            opt.assert(&sum.eq(&target_val));
        }
    }

    let total = button_vars.into_iter().reduce(|a, b| a + b).unwrap();
    opt.minimize(&total);

    if opt.check(&[]) == SatResult::Sat {
        let model = opt.get_model().unwrap();
        let result = model.eval(&total, true).unwrap().as_u64().unwrap();
        Ok(result as usize)
    } else {
        Err(anyhow::Error::msg("No valid button Presses"))
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
