use crate::utilities::UniversalData;
use toml::Value as toml_val;
use serde_json::Value as json_val;

pub fn toml_reader(path:&str)-> UniversalData {
    let content =
    std::fs::read_to_string(path).unwrap();
    let toml: toml_val =
    toml::from_str(&content).unwrap();
    let json_safe = serde_json::to_value(toml).unwrap_or_else(|_| {
        json_val::String("unsupported value".to_string())
    });
    UniversalData::StructJson(json_safe)
}
