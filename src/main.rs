use chrono::NaiveDate;
use clap::Parser;
use csv::Reader;
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
    let relevant_headers = [
        "Project Name",
        "Ticket No",
        "Summary",
        "Hr. Spent",
        "Log Date & Time",
    ];

    let records = csv_reader
        .into_records()
        .filter_map(|record| record.ok())
        .collect::<Vec<_>>();

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
            let date_time_column = &row[3];
            let date_column = date_time_column.split(" ").collect::<Vec<_>>()[0].to_owned();

            row[3] = date_column;
            row
        })
        .sorted_by(|record_a, record_b| {
            let date_record_a = NaiveDate::parse_from_str(&record_a[3], "%d-%b-%Y")
                .expect("Invalid date encountered");
            let date_record_b = NaiveDate::parse_from_str(&record_b[3], "%d-%b-%Y")
                .expect("Invalid date encountered");

            date_record_a.cmp(&date_record_b)
        })
        .group_by(|row| row[3].clone())
        .into_iter()
        .map(|(_, group)| group.into_iter().collect::<Vec<_>>())
        .map(|row| {
            row.into_iter()
                .group_by(|row| row[1].clone())
                .into_iter()
                .map(|(_, group)| {
                    group
                        .into_iter()
                        .fold(Vec::<Vec<String>>::new(), |mut acc, inner_vec| {
                            if let Some(last) = acc.last_mut() {
                                // Parse the last field as a float
                                if let Ok(last_value) = last[4].parse::<f32>() {
                                    // Add the parsed value to the sum
                                    if let Ok(new_value) = inner_vec[4].parse::<f32>() {
                                        last[4] = (last_value + new_value).to_string();
                                    }
                                }
                            }

                            // If the accumulator is empty or the last field couldn't be parsed, add the inner_vec as is
                            if acc.is_empty() || acc.last().unwrap()[4] == inner_vec[4] {
                                acc.push(inner_vec);
                            }

                            acc
                        })
                })
                .into_iter()
                .flatten()
                .map(|row| {
                    let mut new_row = vec![];
                    new_row.push(row[3].clone());
                    let formatted_task = format!("{}: [{}] {}", row[0], row[1], row[2]);
                    new_row.push(formatted_task);
                    new_row.push(row[4].clone());
                    new_row
                })
                .fold(Vec::<String>::new(), |mut acc, row| {
                    println!("inner print");
                    println!("{:?}", row);
                    println!();

                    // A similar pattern to this needs to be used here
                    // if let Some(last) = acc.last_mut() {
                    //     // Parse the last field as a float
                    //     if let Ok(last_value) = last[4].parse::<f32>() {
                    //         // Add the parsed value to the sum
                    //         if let Ok(new_value) = inner_vec[4].parse::<f32>() {
                    //             last[4] = (last_value + new_value).to_string();
                    //         }
                    //     }
                    // }

                    // // If the accumulator is empty or the last field couldn't be parsed, add the inner_vec as is
                    // if acc.is_empty() || acc.last().unwrap()[4] == inner_vec[4] {
                    //     acc.push(inner_vec);
                    // }

                    // acc

                    acc
                });
        })
        .collect::<Vec<_>>();

    for record in relevant_records {
        println!();
        println!("{:?}", record)
        // for column in record {
        //     println!("{:?}", column);
        // }
    }

    Ok(())
}
