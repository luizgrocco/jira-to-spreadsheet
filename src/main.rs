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

    let records = csv_reader
        .into_records()
        .filter_map(|record| record.ok())
        .collect::<Vec<_>>();

    let relevant_records = records.iter().map(|record| {
        record
            .into_iter()
            .enumerate()
            .filter_map(|(column_index, column)| {
                if relevant_headers.contains(&headers.get(column_index)?) {
                    Some(column)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    });

    for record in relevant_records {
        let date_column = &record[3];

        let day = date_column.split(",").collect::<Vec<_>>();
        println!("{:?}", day);
    }

    Ok(())
}
