use crate::utilities::UniversalData;
use toml::Value as toml_val;
pub fn toml_writer(data: &UniversalData, path: &str) {
    if let UniversalData::StructToml(obj) = data {
        if let toml_val::Array(arr) = obj {
            let mut output: String = String::new();
            for item in arr {
                if let toml_val::Table(tbl) = item {
                    output.push_str("[[items]]\n");
                    output.push_str(&toml::to_string_pretty(&tbl).unwrap());
                    output.push('\n');
                }
            }
            std::fs::write(path, output).unwrap();
        } else {
            std::fs::write(path, toml::to_string_pretty(obj).unwrap()).unwrap();
        }
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
