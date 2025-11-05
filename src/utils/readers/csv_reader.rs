use crate::utilities::UniversalData;

pub fn csv_reader(path: &str) -> UniversalData {
    // Reads into enum type UniversalData::Table
    let mut read = csv::Reader::from_path(path).expect("ERROR: Failed to read input file.");
    let headers: Vec<String> = read
        .headers()
        .expect("ERROR: Failed to read headers.")
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
