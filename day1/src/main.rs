use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let reader = BufReader::new(File::open("input")?);
    let mut calories = vec![0];

    for line in reader.lines() {
        match line.context("error reading from file")?.parse::<i32>() {
            Ok(number) => calories[0] += number,
            Err(_) => calories.insert(0, 0),
        }
    }

    calories.sort_unstable();
    println!(
        "calories: {:?}",
        calories
            .get(calories.len() - 3..)
            .unwrap()
            .iter()
            .sum::<i32>()
    );

    Ok(())
}
