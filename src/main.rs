use chrono::NaiveDate;
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

    let mut records = csv_reader
        .into_records()
        .filter_map(|record| record.ok())
        .collect::<Vec<_>>();

    let total = records.pop().unwrap();

    let relevant_records = records
        .iter()
        .map(|record| {
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
        })
        .map(|mut row| {
            let date_column = &row[3];

            let date_arr = date_column.split(",").collect::<Vec<_>>();

            let month_day = date_arr[0];
            let year = date_arr[1];

            let month_day = month_day.split(" ").collect::<Vec<_>>();

            let month = month_day[0];
            let padded_day = if month_day[1].len() == 1 {
                format!("0{}", month_day[1])
            } else {
                month_day[1].to_owned()
            };

            let stringified_date = format!("{}-{}-{}", month, padded_day, year);

            let date = NaiveDate::parse_from_str(&stringified_date, "%B-%d- %Y")
                .unwrap()
                .to_string();

            println!("{:?}", date);

            row[3] = Box::leak(date.into_boxed_str());
            row
        })
        .collect::<Vec<_>>();

    for record in relevant_records {
        println!("{:?}", record);
    }

    Ok(())
}
