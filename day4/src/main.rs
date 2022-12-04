use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs::File;
use std::io::Seek;
use std::io::{BufRead, BufReader};
use std::ops::Range;
use std::str::FromStr;

fn main() -> Result<()> {
    let file_name = "input";
    let mut file =
        File::open(&file_name).context(format!("while opening file '{}'", &file_name))?;

    println!("part1: {}", part1(BufReader::new(&file))?);

    file.rewind()?;
    println!("part2: {}", part2(BufReader::new(&file))?);
    Ok(())
}

fn parse_range(range: &str) -> Result<Range<u32>> {
    let mut tokens = range.splitn(2, "-");
    let start = u32::from_str(tokens.next().unwrap())?;
    let end = u32::from_str(tokens.next().unwrap())?;

    Ok(Range {
        start: start,
        end: end + 1, // range is defined: [start;end[ ... but we want [start;end]
    })
}

fn part1<T>(reader: BufReader<T>) -> Result<u32>
where
    T: std::io::Read,
{
    let mut cnt = 0;
    for line in reader.lines() {
        let line = line.unwrap().to_string();
        let mut sections = line.splitn(2, ",");

        let sections1: HashSet<u32> = HashSet::from_iter(parse_range(sections.next().unwrap())?);
        let sections2: HashSet<u32> = HashSet::from_iter(parse_range(sections.next().unwrap())?);

        if sections1.is_superset(&sections2) || sections2.is_superset(&sections1) {
            cnt += 1
        }
    }

    Ok(cnt)
}

fn part2<T>(reader: BufReader<T>) -> Result<u32>
where
    T: std::io::Read,
{
    let mut cnt = 0;
    for line in reader.lines() {
        let line = line.unwrap().to_string();
        let mut sections = line.splitn(2, ",");

        let sections1: HashSet<u32> = HashSet::from_iter(parse_range(sections.next().unwrap())?);
        let sections2: HashSet<u32> = HashSet::from_iter(parse_range(sections.next().unwrap())?);

        if sections1.intersection(&sections2).count() > 0
            || sections2.intersection(&sections1).count() > 0
        {
            cnt += 1
        }
    }

    Ok(cnt)
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use std::ops::Range;

    #[test]
    fn parse_range() {
        let range = super::parse_range("2-6").unwrap();

        assert_eq!(range, Range { start: 2, end: 6 });
    }

    #[test]
    fn part1() {
        let reader = BufReader::new(
            r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#
            .as_bytes(),
        );

        let result = super::part1(reader).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn part2() {
        let reader = BufReader::new(
            r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#
            .as_bytes(),
        );

        let result = super::part2(reader).unwrap();
        assert_eq!(result, 4);
    }
}
