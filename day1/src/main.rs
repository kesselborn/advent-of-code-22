use anyhow::Result;
use std::fs::File;
use std::io::Read;

fn main() -> Result<()> {
    let mut f = File::open("input")?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    let mut max = 0;
    let mut cur = 0;

    for line in buffer.split("\n").into_iter() {
        if line == "" {
            if cur > max {
                max = cur
            }
            cur = 0
        } else {
            let num: i32 = line.parse::<i32>()?;
            cur += num;
        }
    }

    println!("max calories: {}", max);

    Ok(())
}
