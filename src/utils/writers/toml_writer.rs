use crate::{utilities::UniversalData, utils::BetterExpect};
use toml::Value as toml_val;

pub fn toml_writer(data: &UniversalData, path: &str) {
    // Check if input data is a table or struct-based (like JSON and TOML) data.
    if let UniversalData::Structured(non_toml) = data {
        // Serialize to TOML
        let toml_ser = toml_val::try_from(non_toml)
            .unwrap_or_else(|_| toml_val::String("Unsupported Value".to_string()));
        // First, check if the data has a top level array to which TOML doesn't support to handle it
        if let toml_val::Array(arr) = toml_ser {
            let mut output: String = String::new();
            arr.iter().for_each(|item| {
                if let toml_val::Table(obj) = item {
                    output.push_str("[[Array]]\n");
                    output.push_str(&toml::to_string_pretty(&obj).unwrap_or_default());
                    output.push('\n');
                }
            });
            // Write into the file.
            std::fs::write(path, output.trim_end())
                .better_expect("ERROR: Failed to write final file.");
        // If it doesn't have a top level Array, it will just write into the file.
        } else {
            std::fs::write(path, toml::to_string_pretty(&toml_ser).unwrap_or_default())
                .better_expect("ERROR: Failed to write into output file.");
        }
    // If table based, write into the file by making keys of the TOML tables (objects) the headers (column names) of the table.
    } else if let UniversalData::Table { headers, rows } = data {
        // Iterator chain for writing into the file by using the `.zip()` method on keys (table headers) and values.
        let mut toml_str: String = String::new();
        rows.iter().for_each(|row| {
            toml_str.push_str("[[Rows]]\n");
            headers.iter().zip(row.iter()).for_each(|(h, v)| {
                h.trim().to_string();
                if h.bytes().any(|c| !(c.is_ascii_alphanumeric() || c == b'_')) {
                    toml_str.push_str(&format!("\"{}\" = \"{}\"\n", h, v));
                } else {
                    toml_str.push_str(&format!("{} = \"{}\"\n", h, v));
                }
            });
            toml_str.push('\n');
        });
        std::fs::write(path, toml_str.trim_end())
            .better_expect("ERROR: Failed to write into output file.");
    }
}
