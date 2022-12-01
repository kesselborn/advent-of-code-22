use anyhow::Result;
use std::fs::File;
use std::io::Read;

fn main() -> Result<()> {
    let mut buffer = String::new();
    File::open("input")?.read_to_string(&mut buffer)?;

    let mut calories = vec![];
    let mut cur = 0;

    for line in buffer.split("\n").into_iter() {
        if line == "" {
            calories.push(cur);
            cur = 0
        } else {
            let num: i32 = line.parse::<i32>()?;
            cur += num;
        }
    }

    calories.sort_unstable();
    // calories.reverse();
    println!(
        "max calories: {:?}",
        calories
            .get(calories.len() - 3..)
            .unwrap()
            .iter()
            .sum::<i32>()
    );

    Ok(())
}
