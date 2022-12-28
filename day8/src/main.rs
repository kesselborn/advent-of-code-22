use anyhow::{Context, Result};
use day8::forrest::Forrest;
use env_logger;
use log::LevelFilter;
use std::io::Read;
use std::{env, fs};

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

    let forrest: Forrest = input.parse()?;

    println!("part1: {}", forrest.num_of_visible_trees());

    Ok(())
}
