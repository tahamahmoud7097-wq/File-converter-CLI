use crate::utilities::UniversalData;
use toml::Value as toml_val;
pub fn toml_writer(data: &UniversalData, path: &str) {
    if let UniversalData::StructToml(obj) = data {
        std::fs::write(path, toml::to_string_pretty(obj).unwrap()).unwrap();
    } else if let UniversalData::Table { headers, rows } = data {
        let toml_arr: Vec<toml_val> = rows
            .iter()
            .map(|row| {
                let obj: Vec<_> = headers
                    .iter()
                    .zip(row.iter())
                    .map(|(h, v)| (h.clone(), toml::Value::String(v.clone())))
                    .collect();
                toml_val::try_from(obj).unwrap()
            })
            .collect();
        std::fs::write(path, toml::to_string_pretty(&toml_arr).unwrap()).unwrap();
    }
}
