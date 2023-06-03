use clap::Parser;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let csv_file = std::fs::read_to_string(&args.path)?;

    for line in csv_file.lines() {
        println!("{}", line);
    }
    Ok(())
}
