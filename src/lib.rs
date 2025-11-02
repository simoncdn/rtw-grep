pub mod config;

use std::{error::Error, fs};
use crate::config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let read_file = fs::read_to_string(config.file_path)?;

    println!("file: {:?}", read_file);

    Ok(())
}
