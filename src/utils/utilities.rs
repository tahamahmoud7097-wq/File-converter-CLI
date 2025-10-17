use serde_json::Value as json_val;
use toml::Value as toml_val;

pub enum UniversalData {
    Table {
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
    },
    StructJson(json_val),
    StructToml(toml_val),
}
