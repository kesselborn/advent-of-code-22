use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let file = "input";
    let reader =
        BufReader::new(File::open(file).context(format!("while opening file '{}'", &file))?);

    let result = reader
        .lines()
        .fold(0, |acc, line| match line.unwrap().as_ref() {
            "A X" => acc + 3 + 1, // Rock     vs. Rock
            "B X" => acc + 0 + 1, // Paper    vs. Rock
            "C X" => acc + 6 + 1, // Scissors vs. Rock

            "A Y" => acc + 6 + 2, // Rock     vs. Paper
            "B Y" => acc + 3 + 2, // Paper    vs. Paper
            "C Y" => acc + 0 + 2, // Scissors vs. Paper

            "A Z" => acc + 0 + 3, // Rock     vs. Scissors
            "B Z" => acc + 6 + 3, // Paper    vs. Scissors
            "C Z" => acc + 3 + 3, // Scissors vs. Scissors
            _ => acc,
        });

    println!("result: {}", result);

    Ok(())
}
