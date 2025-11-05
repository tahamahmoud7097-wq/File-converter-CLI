use crate::utilities::UniversalData;
use toml::Value as toml_val;

pub fn toml_writer(data: &UniversalData, path: &str) {
    // Check if input data is a table or struct-based (like JSON and TOML) data.
    if let UniversalData::StructToml(obj) = data {
        // First, check if the data has a top level array to which TOML doesn't support to handle it by adding [[Array]] to the top of each object.
        if let toml_val::Array(arr) = obj {
            let mut output: String = String::new();
            for item in arr {
                if let toml_val::Table(obj) = item {
                    output.push_str("[[Array]]\n");
                    output.push_str(&toml::to_string_pretty(&obj).unwrap_or_default());
                    output.push('\n');
                }
            }
            // Write into the file.
            std::fs::write(path, output).expect("ERROR: Failed to write final file.");
        // If it doesn't have a top level Array, it will just write into the file.
        } else {
            std::fs::write(path, toml::to_string_pretty(obj).unwrap_or_default())
                .expect("ERROR: Failed to write into output file.");
        }
    // If table based, write into the file by making keys of the TOML tables (objects) the headers (column names) of the table.
    } else if let UniversalData::Table { headers, rows } = data {
        // Iterator chain for writing into the file by using the `.zip()` method on keys (table headers) and values.
        let toml_arr: Vec<toml_val> = rows
            .iter()
            .map(|row| {
                let obj: Vec<_> = headers
                    .iter()
                    .zip(row.iter())
                    .map(|(h, v)| (h.clone(), toml::Value::String(v.clone())))
                    .collect();
                toml_val::try_from(obj).expect("ERROR: Failed to serialize object.")
            })
            .collect();
        std::fs::write(path, toml::to_string_pretty(&toml_arr).unwrap_or_default())
            .expect("ERROR: Failed to write into output file.");
    }
}
