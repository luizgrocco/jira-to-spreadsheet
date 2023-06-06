use clap::Parser;
use csv::Reader;
use std::error::Error;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    path: PathBuf,
}

// fn main() -> Result<(), Box<dyn Error>> {
//     let args = Cli::parse();

//     let mut csv_reader = Reader::from_path(&args.path)?;

//     let headers = csv_reader.headers()?.clone();
//     let relevant_headers = vec!["Project", "Issue", "Summary", "Time spent"];
//     let records = csv_reader.records();

//     let relevant_columns: Vec<Vec<String>> = headers
//         .iter()
//         .enumerate()
//         .filter_map(|(_, header)| {
//             if relevant_headers.contains(&header) {
//                 Some(header)
//             } else {
//                 None
//             }
//         })
//         .map(|(index, _)| {
//             csv_reader
//                 .records()
//                 .map(|record| record.map(|row| row[index].to_owned()))
//                 .collect::<Result<Vec<_>, _>>()
//         })
//         .collect::<Result<Vec<_>, _>>()?;

//     for record in relevant_columns {
//         println!("{:?}", record);
//     }

//     Ok(())
// }

// fn main() -> Result<(), Box<dyn Error>> {
//     let args = Cli::parse();

//     let mut csv_reader = Reader::from_path(&args.path)?;

//     let headers = csv_reader.headers()?.into_iter();
//     let relevant_headers = vec!["Project", "Issue", "Summary", "Time spent"];

//     let index_array: Vec<usize> = headers
//         .enumerate()
//         .filter(|(_, header)| relevant_headers.contains(header))
//         .map(|(index, _)| index)
//         .collect();

//     let relevant_records = csv_reader.records().map(|record| {
//         record
//             .into_iter()
//             .enumerate()
//             .filter(|(column_index, _)| index_array.contains(column_index))
//             // .map(|(_, column)| column)
//             .collect::<Vec<_>>()
//     });

//     for record in relevant_records {
//         println!("{:?}", record);
//     }

//     Ok(())
// }

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let mut csv_reader = Reader::from_path(&args.path)?;

    let headers = csv_reader.headers()?.clone();

    let relevant_headers = vec!["Project", "Issue", "Summary", "Time spent"];

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
    });

    for record in relevant_records {
        for column in record {
            println!("{:?}", column);
        }
    }

    Ok(())
}
