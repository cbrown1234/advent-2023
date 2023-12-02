use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    let file = File::open("input_data.txt")?;
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
    Ok(())
}
