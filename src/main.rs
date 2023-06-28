use chrono::NaiveDate;
use clap::Parser;
use csv::Reader;
use humantime::parse_duration;
use itertools::Itertools;
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
                        Some(String::from(column))
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

            row[3] = date;
            row
        })
        .group_by(|row| row[3].clone())
        .into_iter()
        .map(|(_, group)| group.into_iter().collect::<Vec<_>>())
        .map(|groups| {
            groups
                .into_iter()
                .map(|mut row| {
                    let duration = parse_duration(&row[4]).unwrap().as_secs();
                    let duration = duration.to_string();
                    row[4] = duration;
                    row
                })
                .group_by(|row| row[1].clone())
                .into_iter()
                .map(|(_, grouped_row)| {
                    grouped_row
                        .into_iter()
                        .fold(None, |acc: Option<Vec<String>>, row| {
                            return match acc {
                                Some(mut acc) => {
                                    let current_hours = acc[4].parse::<u32>().unwrap();
                                    let hours = row[4].parse::<u32>().unwrap();
                                    let hours_total = current_hours + hours;
                                    let stringified_hours = hours_total.to_string();
                                    acc[4] = stringified_hours;
                                    Some(acc)
                                }
                                None => Some(row),
                            };
                        })
                })
                .map(|row| row.unwrap())
                .collect::<Vec<_>>()
        }) // TODO: process groups
        .collect::<Vec<_>>();

    for record in relevant_records {
        println!("");
        for column in record {
            println!("{:?}", column);
        }
    }

    Ok(())
}
