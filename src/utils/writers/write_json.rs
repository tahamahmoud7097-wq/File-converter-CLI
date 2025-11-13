use crate::{
    utilities::{UniversalData, Vals},
    utils::BetterExpect,
};
use serde_json::{Map, Value as JsonVal, json};

pub fn write_json(data: &UniversalData, path: &str) {
    // Check if input data is struct, key-value based (like JSON and TOML) or table (like CSV)
    if let UniversalData::Structured(Vals::Json(json)) = data {
        std::fs::write(path, serde_json::to_string_pretty(json).unwrap_or_default())
            .better_expect("ERROR: Failed to write into output file.");
    // If table based, uses the `.zip()` method to bind table headers (column names) as keys to their values in the rows to form key-value pairs for serde_json to serialize
    } else if let UniversalData::Table { headers, rows } = data {
        let json_arr: Vec<JsonVal> = rows
            .iter()
            .map(|row| {
                let mut obj: Map<String, JsonVal> = Map::new();
                headers.iter().zip(row.iter()).for_each(|(h, v)| {
                    obj.insert(h.clone(), json!(v));
                });
                JsonVal::Object(obj)
            })
            .collect();
        std::fs::write(
            path,
            serde_json::to_string_pretty(&json_arr).unwrap_or_default(),
        )
        .better_expect("ERROR: Failed to write into output file.");
    }
}
