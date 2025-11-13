use crate::{
    utilities::{UniversalData, Vals},
    utils::BetterExpect,
};
use serde_json::Value as JsonVal;
use toml::Value as TomlVal;

pub fn json_reader(path: &str) -> UniversalData {
    // Reads then converts to TOML format
    let content = std::fs::read_to_string(path).better_expect("ERROR: Failed to read input file.");
    let json: JsonVal =
        serde_json::from_str(&content).better_expect("ERROR: Failed to deserialize file.");
    let toml_safe = TomlVal::try_from(json)
        .unwrap_or_else(|_| TomlVal::String("Unsupported Value".to_string()));
    UniversalData::Structured(Vals::Toml(toml_safe))
}
