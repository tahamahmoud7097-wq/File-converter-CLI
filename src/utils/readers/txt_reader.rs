use crate::utilities::UniversalData;
use serde_json::Value as json_val;
use toml::Value as toml_val;

pub fn read_from_txt(path: &str, output: &str) -> UniversalData {
    let content = std::fs::read_to_string(path).unwrap();
    match output {
        "csv" => {
            let rows: Vec<Vec<String>> = content
                .lines()
                .map(|line| line.split_whitespace().map(|s| s.to_string()).collect())
                .collect();
            let headers: &Vec<String> = &rows[1];
            UniversalData::Table {
                headers: headers.to_vec(),
                rows: rows[1..rows.len()].to_vec(),
            }
        }
        "json" => {
            let objs: json_val = serde_json::from_str(&content).unwrap();
            UniversalData::StructJson(objs)
        }
        "toml" => {
            let tomls: toml_val = toml::from_str(&content).unwrap();
            UniversalData::StructToml(tomls)
        }
        _ => panic!("Unsupported output format: {output}"),
    }
}
