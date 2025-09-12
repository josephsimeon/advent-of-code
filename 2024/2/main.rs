/// @file   main.rs
/// @brief  Advent of Code Day 2 for 2024
/// @author Joseph Simeon
/// @date   22-06-2025 06:53:48

use std::fs::File;
use std::io::{BufRead, BufReader};

fn check_safe_levels (levels: &Vec<i32>) -> bool
{
    if levels.len() <= 1 { return true };

    // check if all increasing
    let mut ascending: bool = true;
    'checking_ascending: for i in 0..(levels.len() - 1)
    {
        let curr = levels[i];
        let next = levels[i + 1];
        
        if curr >= next
        {
            ascending = false;
            break 'checking_ascending;
        }
    }

    // check if all decreasing
    let mut descending: bool = true;
    'checking_descending: for i in 0..(levels.len() - 1)
    {
        let curr = levels[i];
        let next = levels[i + 1];
        
        if curr <= next
        {
            descending = false;
            break 'checking_descending;
        }
    }

    // check if within threshold
    let mut within_threshold: bool = true;
    'checking_threshold: for i in 0..(levels.len() - 1)
    {
        let curr = levels[i];
        let next = levels[i + 1];
        let abs = (curr - next).abs();
        
        if !(1..=3).contains(&abs)
        {
            within_threshold = false;
            break 'checking_threshold
        }
    }

    (ascending || descending) && within_threshold
}

fn check_safe_levels_w_tolerance (levels: &Vec<i32>) -> bool
{
    for i in 0..levels.len()
    {
        let mut toleranced_levels = levels.clone();
        toleranced_levels.remove(i);
        if check_safe_levels(&toleranced_levels) { return true };
    }

    false
}

fn solution (file: &str) -> (i32, i32)
{
    let f = BufReader::new(File::open(file).expect("Unable to find file."));

    let mut first_pass: i32 = 0;
    let mut second_pass: i32 = 0;

    for line in f.lines()
    {
        let nums: Vec<i32> = line
            .expect("Unable to read line")
            .split_whitespace()                     // Iterator through chars of &str by splitting whitespace
            .map(|x| x.parse::<i32>().unwrap())     // Parse each item to <i32> by unwrapping Result<i32, _>
            .collect();                             // Collect into vector

        if check_safe_levels(&nums) { first_pass += 1 }
            else { if check_safe_levels_w_tolerance(&nums) { second_pass += 1 }};
    }

    (first_pass, first_pass + second_pass)
}

fn main ()
{
    println!("{:?}", solution("./puzzle.txt"));
}

#[test]
fn test ()
{
    println!("{:?}", solution("./example.txt"));
}
