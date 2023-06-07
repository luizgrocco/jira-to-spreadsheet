use clap::Parser;
use csv::Reader;
use std::error::Error;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    path: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let mut csv_reader = Reader::from_path(&args.path)?;

    let headers = csv_reader.headers()?.clone();
    let relevant_headers = ["Project", "Issue", "Summary", "Time spent", "Started"];

    let relevant_records = csv_reader.records().map(|record| {
        record
            .unwrap()
            .into_iter()
            .enumerate()
            .filter_map(|(column_index, column)| {
                if relevant_headers.contains(&headers.get(column_index)?) {
                    Some(column)
                } else {
                    None
                }
            })
            .map(|column| column.to_owned())
            .collect::<Vec<_>>()
    });

    for record in relevant_records {
        // for column in record {
        println!("{:?}", record);
        // }
    }

    Ok(())
}
