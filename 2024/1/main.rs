/// @file   main.rs
/// @brief  Advent of Code Day 1 for 2024
/// @author Joseph Simeon
/// @date   12/3/2025 1:57 am

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = BufReader::new(File::open("./puzzle.txt")
                            .expect("Unable to open file"));

    let mut x: Vec<u32> = Vec::new();
    let mut y: Vec<u32> = Vec::new();

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        for (j, word) in line.split_whitespace().enumerate() {
            match j {
                0 => x.push(word.parse().unwrap()),
                1 => y.push(word.parse().unwrap()),
                _ => continue,
            }
        }
    }

    x.sort();
    y.sort();

    let mut difference_sum: u32 = 0;
    let mut similiarity_sum: u32 = 0;

    for (x_num, y_num) in x.iter().zip(y.iter()) {
        difference_sum += (*x_num as i32 - *y_num as i32).abs() as u32;
        similiarity_sum += *x_num as u32 * y.iter().filter(|n| *n == x_num).count() as u32;
    }

    println!("Sum of differences: {}", difference_sum);
    println!("Sum of similiarity: {}", similiarity_sum);
}
