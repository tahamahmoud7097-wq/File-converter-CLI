use crate::utilities::UniversalData;

pub fn csv_writer(data: &UniversalData, path: &str) {
    if let UniversalData::Table{ headers, rows } = data {
        let mut wtr = csv::Writer::from_path(path).unwrap();
        wtr.write_record(headers).unwrap();
        for row in rows {
            wtr.write_record(row).unwrap();
        }
        wtr.flush().unwrap();
    } else {
        panic!("csv only supports tables.");
    }
}
