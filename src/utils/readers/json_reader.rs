use crate::{utilities::UniversalData, utils::BetterExpect};
use serde_json::Value as json_val;
use toml::Value as toml_val;

pub fn json_reader(path: &str) -> UniversalData {
    // Reads then converts to TOML format
    let content = std::fs::read_to_string(path).better_expect("ERROR: Failed to read input file.");
    let json: json_val =
        serde_json::from_str(&content).better_expect("ERROR: Failed to deserialize file.");
    let toml_safe = toml_val::try_from(json)
        .unwrap_or_else(|_| toml_val::String("Unsupported Value".to_string()));
    UniversalData::StructToml(toml_safe)
}
