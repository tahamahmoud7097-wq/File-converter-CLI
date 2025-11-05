use serde_json::Value as json_val;
use toml::Value as toml_val;
// Main enum for a universal data type so all readers and writers can share one type.
pub enum UniversalData {
    Table {
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
    },
    StructJson(json_val),
    StructToml(toml_val),
}
