use chrono::NaiveDate;
use clap::Parser;
use csv::Reader;
use itertools::Itertools;
use rust_xlsxwriter::{Format, FormatAlign, Workbook};
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
                .fold(
                    (String::new(), String::new(), 0.0),
                    |(mut date_acc, mut tasks_summary_acc, mut hours_acc), row| {
                        let row_date = row[0].clone();
                        let row_task = row[1].clone();
                        let row_hours = row[2].parse::<f64>().unwrap();

                        if date_acc.is_empty() {
                            date_acc = row_date;
                        }
                        if tasks_summary_acc.is_empty() {
                            tasks_summary_acc = row_task;
                        } else {
                            tasks_summary_acc = format!("{}\r{}", tasks_summary_acc, row_task);
                        }
                        hours_acc = hours_acc + row_hours;

                        (date_acc, tasks_summary_acc, hours_acc)
                    },
                )
        })
        .map(|(date_acc, tasks_summary_acc, hours_acc)| {
            (
                NaiveDate::parse_from_str(&date_acc, "%d-%b-%Y").unwrap(),
                tasks_summary_acc,
                hours_acc * 3600.0,
            )
        })
        .collect::<Vec<_>>();

    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    let date_format = Format::new()
        .set_align(FormatAlign::Center)
        .set_num_format("dd/mm/yyyy");
    let task_summary_format = Format::new().set_align(FormatAlign::Left);
    let hours_format = Format::new()
        .set_align(FormatAlign::Right)
        .set_num_format("[hh]:mm:ss");

    for (row_index, (date, task_summary, hours)) in relevant_records.iter().enumerate() {
        worksheet
            .write_with_format(row_index as u32, 0, date, &date_format)
            .expect(&format!("failed to write date column in row {}", row_index));
        worksheet
            .write_with_format(row_index as u32, 1, task_summary, &task_summary_format)
            .expect(&format!(
                "failed to write task summary in row {}",
                row_index
            ));
        worksheet
            .write_number_with_format(row_index as u32, 2, *hours, &hours_format)
            .expect(&format!(
                "failed to write hours column in row {}",
                row_index
            ));
    }

    // TODO: Last row with hour totals: worksheet.merge_range(1, 1, 1, 2, "Merged cells", &format)?;

    let export_path = format!(
        "{}/results.xlsx",
        args.path.parent().unwrap().to_str().unwrap()
    );

    worksheet
        .set_column_width(1, 100)
        .expect("failed setting column width");
    workbook.save(export_path).expect("failed to save file!");
    Ok(())
}
