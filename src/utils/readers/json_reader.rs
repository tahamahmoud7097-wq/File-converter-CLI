use crate::utilities::UniversalData;
use serde_json::Value as json_val;
use toml::Value as toml_val;

pub fn json_reader(path: &str) -> UniversalData {
    let content = std::fs::read_to_string(path).unwrap();
    let json: json_val = serde_json::from_str(&content).unwrap();
    let toml_safe = match toml_val::try_from(json) {
        Ok(v) => v,
        Err(_) => toml_val::String("Unsupported Value".to_string()),
    };
    UniversalData::StructToml(toml_safe)
}
