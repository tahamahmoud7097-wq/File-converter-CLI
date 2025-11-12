use colored::Colorize;
use serde_json::Value as json_val;
use toml::Value as toml_val;

// Main enum for a universal data type so all readers and writers can share one type

pub enum UniversalData {
    Table {
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
    },
    StructJson(json_val),
    StructToml(toml_val),
}

// Custom better expect trait for better error messages without duping code

pub trait BetterExpect<T> {
    fn better_expect(self, msg: &str) -> T;
}

// impl for Result which matches the value to Ok to return the value or print the error msg in red if Err
impl<T, E> BetterExpect<T> for Result<T, E> {
    fn better_expect(self, msg: &str) -> T {
        match self {
            Ok(v) => v,
            Err(_) => {
                eprintln!("{}", msg.red().bold());
                std::process::exit(1);
            }
        }
    }
}

// impl for Option to match the value for Some to return the actual value and if None prints error msg in red

impl<T> BetterExpect<T> for Option<T> {
    fn better_expect(self, msg: &str) -> T {
        match self {
            Some(v) => v,
            None => {
                eprintln!("{}", msg.red().bold());
                std::process::exit(1);
            }
        }
    }
}
