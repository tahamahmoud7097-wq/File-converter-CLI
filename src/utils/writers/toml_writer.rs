use crate::utilities::UniversalData;

pub fn toml_writer(data: &UniversalData, path: &str) {
    if let UniversalData::StructToml(toml) = data {
        std::fs::write(path, 
        serde_json::to_string_pretty(toml)
        .unwrap()).unwrap();
    } else {
        panic!("toml only supports structured data");
    }
}
