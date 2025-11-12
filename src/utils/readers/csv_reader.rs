use crate::{utilities::UniversalData, utils::BetterExpect};

pub fn csv_reader(path: &str) -> UniversalData {
    // Reads into enum type UniversalData::Table
    let mut read = csv::Reader::from_path(path).better_expect("ERROR: Failed to read input file.");
    let headers: Vec<String> = read
        .headers()
        .better_expect("ERROR: Failed to read headers. Make sure CSV file has headers for conversions to work.")
        .iter()
        .map(|h| h.to_string())
        .collect();
    let rows: Vec<Vec<String>> = read
        .records()
        .map(|r| {
            r.unwrap_or_default()
                .iter()
                .map(|s| s.to_string())
                .collect()
        })
        .collect();
    UniversalData::Table { headers, rows }
}
