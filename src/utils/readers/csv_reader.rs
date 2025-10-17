use crate::utilities::UniversalData;

pub fn csv_reader(path: &str) -> UniversalData {
    let mut read = csv::Reader::from_path(path).unwrap();
    let headers: Vec<String> = read
        .headers()
        .unwrap()
        .iter()
        .map(|h| h.to_string())
        .collect();
    let rows: Vec<Vec<String>> = read
        .records()
        .map(|r| r.unwrap().iter().map(|s| s.to_string()).collect())
        .collect();
    UniversalData::Table { headers, rows }
}
