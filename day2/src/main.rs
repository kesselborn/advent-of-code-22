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
            // Rock: 1
            // Paper: 2
            // Scissors: 3
            "A X" => acc + 0 + 3, // lose: Rock vs. Scissors
            "B X" => acc + 0 + 1, // lose: Paper vs. Rock
            "C X" => acc + 0 + 2, // lose: Scissors vs. Paper

            "A Y" => acc + 3 + 1, // draw: Rock     vs. Rock
            "B Y" => acc + 3 + 2, // draw: Paper    vs. Paper
            "C Y" => acc + 3 + 3, // draw: Scissors vs. Scissors

            "A Z" => acc + 6 + 2, // win: Rock     vs. Paper
            "B Z" => acc + 6 + 3, // win: Paper    vs. Scissors
            "C Z" => acc + 6 + 1, // win: Scissors vs. Rock
            _ => acc,
        });

    println!("result: {}", result);

    Ok(())
}
