use crate::utilities::UniversalData;
use serde_json::{Value as JsonVal, json};

pub fn json_writer(data: &UniversalData, path: &str) {
    if let UniversalData::StructJson(json) = data {
        std::fs::write(path, serde_json::to_string_pretty(json).unwrap()).unwrap();
    } else if let UniversalData::Table { headers, rows } = data {
        let json_arr: Vec<JsonVal> = rows
            .iter()
            .map(|row| {
                let obj = headers
                    .iter()
                    .zip(row.iter())
                    .map(|(h, v)| (h.clone(), json!(v)))
                    .collect();
                JsonVal::Object(obj)
            })
            .collect();
        std::fs::write(path, serde_json::to_string_pretty(&json_arr).unwrap()).unwrap();
    }
}
