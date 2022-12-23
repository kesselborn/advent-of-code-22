use day7::session_parser;
use std::io::Read;
use std::{env, fs};

use anyhow::{Context, Result};
use log::{debug, LevelFilter};

fn main() -> Result<()> {
    let file_name = "input";
    let mut file =
        fs::File::open(&file_name).context(format!("while opening file '{}'", &file_name))?;

    let mut log_builder = env_logger::builder();

    if let Ok(debug) = env::var("DEBUG") {
        if debug == "1" {
            log_builder.filter_module(module_path!(), LevelFilter::Debug);
        }
    }
    log_builder.init();

    let mut input = String::new();
    let _ = file.read_to_string(&mut input)?;

    let fs = session_parser::parse_session(&input)?;
    println!("part1: {}", &fs.total_sum_of_all_dirs_smaller_than(100_000));

    let total_fs_size = fs.total_size();
    let necessary_free_space = 30_000_000 - (70_000_000 - total_fs_size);
    println!(
        "part2: {}",
        &fs.smallest_dir_greater_than(necessary_free_space).unwrap()
    );

    debug!("{fs}");

    Ok(())
}
