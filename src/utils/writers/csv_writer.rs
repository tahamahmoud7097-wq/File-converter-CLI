use crate::utilities::UniversalData;

pub fn csv_writer(data: &UniversalData, path: &str) {
    // Check if it's a table before writing.
    if let UniversalData::Table { headers, rows } = data {
        // Open CSV file for writing.
        let mut wtr = csv::Writer::from_path(path).expect("ERROR: Failed to open output CSV file.");
        // Write headers into the file.
        wtr.write_record(headers)
            .expect("ERROR: Failed to write CSV file headers.");
        // Loop for writing rows into the file.
        for row in rows {
            wtr.write_record(row)
                .expect("ERROR: Failed to write CSV file rows.");
        }
        wtr.flush().expect("ERROR: Failed to flush final CSV.");
    // If data is not a table, then panics to prevent broken conversions.
    } else {
        panic!(
            "CSV only supports table based file types or files that can be converted into a table."
        );
    }
}
