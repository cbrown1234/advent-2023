use anyhow::Result;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    let file = File::open("inputs/day1.txt")?;
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();

    let calibration: u32 = lines
        .map(|line| line.expect("able to read file"))
        .filter(|line| !line.is_empty())
        .map(|line| {
            eprintln!("{line}");
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));
            let first = digits.next().expect("first number to exist for every line");
            let last = digits.last().unwrap_or(first);
            (first, last)
        })
        .map(|(first, last)| {
            let number = (10 * first) + last;
            eprintln!("{number}");
            number
        })
        .sum();
    println!("Calibration = {calibration}");

    let numbers = HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    let file = File::open("inputs/day1.txt")?;
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();

    let calibration: u32 = lines
        .map(|line| line.expect("able to read file"))
        .filter(|line| !line.is_empty())
        .map(|line| {
            eprintln!("{line}");
            let positions: Vec<_> = numbers
                .iter()
                .flat_map(|(k, number)| {
                    line.match_indices(k)
                        .map(|(pos, _)| (number.to_owned(), pos))
                })
                .collect();

            let first = positions
                .iter()
                .min_by_key(|(_number, pos)| pos)
                .expect("first number to exist");
            let last = positions
                .iter()
                .max_by_key(|(_number, pos)| pos)
                .unwrap_or(first);
            (first.0, last.0)
        })
        .map(|(first, last)| {
            let number = (10 * first) + last;
            eprintln!("{number}");
            number
        })
        .sum();
    println!("Calibration part 2 = {calibration}");
    Ok(())
}
