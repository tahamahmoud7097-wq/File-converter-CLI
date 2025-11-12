use crate::{utilities::UniversalData, utils::BetterExpect};
use serde_json::Value as json_val;
use toml::Value as toml_val;

pub fn toml_reader(path: &str) -> UniversalData {
    // reads file and then formats to JSON since I still haven't added any other extensions it can convert to, later when I add YAML I will have to use a match statement
    let content = std::fs::read_to_string(path)
        .better_expect("ERROR: Failed to read input file.")
        .trim_end()
        .to_string();
    let toml: toml_val =
        toml::from_str(&content).better_expect("ERROR: Failed to deserialize file.");
    let json_safe = serde_json::to_value(toml)
        .unwrap_or_else(|_| json_val::String("unsupported value".to_string()));
    UniversalData::StructJson(json_safe)
}
