use anyhow::Result;
use std::fs::File;
use std::io::Read;

fn main() -> Result<()> {
    let mut buffer = String::new();
    File::open("input")?.read_to_string(&mut buffer)?;

    let mut calories = vec![0];

    for line in buffer.split("\n").into_iter() {
        match line.parse::<i32>() {
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
