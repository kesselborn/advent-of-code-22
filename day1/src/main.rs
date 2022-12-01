use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let file = "input";
    let reader =
        BufReader::new(File::open(file).context(format!("while opening file '{}'", &file))?);
    let mut calories = vec![0];

    for (cnt, line) in reader.lines().enumerate() {
        match line
            .context(format!("while reading line {} from file '{}'", cnt, file))?
            .parse::<i32>()
        {
            Ok(number) => calories[0] += number,
            Err(_) => calories.insert(0, 0),
        }
    }

    calories.sort_unstable();
    println!(
        "part 1: {}\npart 2: {}",
        calories.last().unwrap(),
        calories
            .get(calories.len() - 3..)
            .unwrap()
            .iter()
            .sum::<i32>()
    );

    Ok(())
}
