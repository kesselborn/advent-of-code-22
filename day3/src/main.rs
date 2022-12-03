use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let file = "input";
    let reader =
        BufReader::new(File::open(file).context(format!("while opening file '{}'", &file))?);

    println!("sum of priorities: {}", sum_of_priorities(reader)?);
    Ok(())
}

fn sum_of_priorities<T>(reader: BufReader<T>) -> Result<u32>
where
    T: std::io::Read,
{
    let result: u32 = reader.lines().fold(0, |acc, line| {
        let line = line.unwrap();
        let (first_compartment_items, second_compartment_items) =
            line.as_str().split_at(&line.len() / 2);

        let first_compartment: HashSet<char> = HashSet::from_iter(first_compartment_items.chars());
        let second_compartment: HashSet<char> =
            HashSet::from_iter(second_compartment_items.chars());

        let duplicate: &char = first_compartment
            .intersection(&second_compartment)
            .last()
            .unwrap();

        let priority = match *duplicate {
            c if c >= 'a' && c <= 'z' => c as u8 - 'a' as u8 + 1,
            c if c >= 'A' && c <= 'Z' => c as u8 - 'A' as u8 + 27,
            _ => 0,
        };

        acc + (priority as u32)
    });

    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    #[test]
    fn sum_of_priorities() {
        let reader = BufReader::new(
            r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#
            .as_bytes(),
        );

        let result = super::sum_of_priorities(reader).unwrap();
        assert_eq!(result, 157);
    }
}
