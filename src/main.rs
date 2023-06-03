use clap::Parser;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    path: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let csv_file = fs::read_to_string(&args.path)?;

    let mut array_of_lines: Vec<Vec<&str>> = Vec::new();

    for line in csv_file.lines() {
        let line_elements: Vec<&str> = line.split(",").collect();
        array_of_lines.push(line_elements);
    }

    println!("{:?}", array_of_lines);

    Ok(())
}
