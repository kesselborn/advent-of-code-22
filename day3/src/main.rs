use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek};

fn main() -> Result<()> {
    let file_name = "input";
    let mut file =
        File::open(&file_name).context(format!("while opening file '{}'", &file_name))?;

    println!("part1: {}", part1(BufReader::new(&file))?);

    file.rewind()?;
    println!("part2: {}", part2(BufReader::new(&file))?);
    Ok(())
}

fn part1<T>(reader: BufReader<T>) -> Result<u32>
where
    T: std::io::Read,
{
    let result: u32 = reader.lines().fold(0, |acc, line| {
        let line = line.unwrap();
        let (first_compartment, second_compartment) = line.as_str().split_at(&line.len() / 2);

        let first_compartment_items: HashSet<char> = HashSet::from_iter(first_compartment.chars());
        let second_compartment_items: HashSet<char> =
            HashSet::from_iter(second_compartment.chars());

        let common_item: &char = first_compartment_items
            .intersection(&second_compartment_items)
            .last()
            .unwrap();

        let priority = calc_priority(common_item);

        acc + (priority as u32)
    });

    Ok(result)
}

fn part2<T>(reader: BufReader<T>) -> Result<u32>
where
    T: std::io::Read,
{
    let mut iterator = reader.lines();

    let mut result = 0;

    loop {
        match (iterator.next(), iterator.next(), iterator.next()) {
            (Some(Ok(first_rucksack)), Some(Ok(second_rucksack)), Some(Ok(third_rucksack))) => {
                let first_rucksack_items: HashSet<char> =
                    HashSet::from_iter(first_rucksack.chars());
                let second_rucksack_items: HashSet<char> =
                    HashSet::from_iter(second_rucksack.chars());
                let third_rucksack_items: HashSet<char> =
                    HashSet::from_iter(third_rucksack.chars());

                let first_and_second_common: String = first_rucksack_items
                    .intersection(&second_rucksack_items)
                    .collect();

                let first_and_second_common_items =
                    HashSet::from_iter(first_and_second_common.chars());

                let common_group_item = first_and_second_common_items
                    .intersection(&third_rucksack_items)
                    .last()
                    .unwrap();

                let priority = calc_priority(common_group_item);

                result += priority as u32;
            }
            _ => break,
        }
    }

    Ok(result)
}

fn calc_priority(duplicate: &char) -> u8 {
    match *duplicate {
        c if c >= 'a' && c <= 'z' => c as u8 - 'a' as u8 + 1,
        c if c >= 'A' && c <= 'Z' => c as u8 - 'A' as u8 + 27,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    #[test]
    fn part1() {
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

        let result = super::part1(reader).unwrap();
        assert_eq!(result, 157);
    }

    #[test]
    fn part2() {
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

        let result = super::part2(reader).unwrap();
        assert_eq!(result, 70);
    }
}
