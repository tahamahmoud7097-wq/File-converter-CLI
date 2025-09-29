use crate::utilities::UniversalData;

pub fn json_writer(data: &UniversalData, path: &str) {
    if let UniversalData::StructJson(json) = data {
        std::fs::write(path, 
        serde_json::to_string_pretty(json)
        .unwrap()).unwrap();
    } else {
        panic!("json only supports structured data");
    }
}
