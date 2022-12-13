use std::fs::File;
use std::io::Read;

use anyhow::{bail, Context, Result};

fn main() -> Result<()> {
    let file_name = "input";
    let mut file =
        File::open(&file_name).context(format!("while opening file '{}'", &file_name))?;

    let mut input = String::new();
    let _ = file.read_to_string(&mut input)?;

    println!("part1: {}", find_marker_pos(&input, 4).unwrap());
    println!("part2: {}", find_marker_pos(&input, 14).unwrap());

    Ok(())
}

fn find_marker_pos(input: &str, marker_size: usize) -> Result<usize> {
    let mut pointer = marker_size;

    while pointer <= input.len() {
        let mut sub_string: Vec<char> = input[pointer - marker_size..pointer].chars().collect();

        sub_string.sort_unstable();
        sub_string.dedup();

        if sub_string.len() == marker_size {
            return Ok(pointer);
        }

        pointer += 1;
    }

    bail!("no marker found in string")
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(
            super::find_marker_pos("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4).unwrap(),
            7
        );
        assert_eq!(
            super::find_marker_pos("bvwbjplbgvbhsrlpgdmjqwftvncz", 4).unwrap(),
            5
        );
        assert_eq!(
            super::find_marker_pos("nppdvjthqldpwncqszvftbrmjlhg", 4).unwrap(),
            6
        );
        assert_eq!(
            super::find_marker_pos("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4).unwrap(),
            10
        );
        assert_eq!(
            super::find_marker_pos("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4).unwrap(),
            11
        );
    }

    #[test]
    fn errors() {
        assert!(super::find_marker_pos("aaaaaaaaaaa", 4).is_err());
        assert!(super::find_marker_pos("a", 4).is_err());
        assert!(super::find_marker_pos("", 4).is_err());
    }

    #[test]
    fn part2() {
        assert_eq!(
            super::find_marker_pos("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14).unwrap(),
            19
        );
        assert_eq!(
            super::find_marker_pos("bvwbjplbgvbhsrlpgdmjqwftvncz", 14).unwrap(),
            23
        );
        assert_eq!(
            super::find_marker_pos("nppdvjthqldpwncqszvftbrmjlhg", 14).unwrap(),
            23
        );
        assert_eq!(
            super::find_marker_pos("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14).unwrap(),
            29
        );
        assert_eq!(
            super::find_marker_pos("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14).unwrap(),
            26
        );
    }
}
