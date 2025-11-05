mod utils;
use std::path::Path;
use utils::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // Check if arguments are valid
    if args.len() != 3 {
        panic!("Invalid usage, correct usage: prog <input> <output>");
    }
    let input: &String = &args[1];
    let output: &String = &args[2];
    // Check if input exists
    if !Path::new(input).exists() {
        panic!("ERROR: Input file doesn't exist.");
    }
    // Check if output exists, if it doesn't then create it
    if !Path::new(output).exists() {
        println!("Output file [{output}] doesn't exist, creating a new file for it...");
        std::fs::File::create(output).expect("ERROR: Couldn't create output file.");
    }
    let input_ext: &str = input.split(".").last().unwrap_or("");
    let output_ext: &str = output.split(".").last().unwrap_or("");
    let now = std::time::Instant::now();
    let data: UniversalData = match input_ext {
        "txt" => txt_reader::read_from_txt(input, output_ext),
        "json" => json_reader::json_reader(input),
        "toml" => toml_reader::toml_reader(input),
        "csv" => csv_reader::csv_reader(input),
        _ => panic!("ERROR: Input extension {input_ext} is not supported"),
    };
    match output_ext {
        "json" => write_json::write_json(&data, output),
        "toml" => toml_writer::toml_writer(&data, output),
        "csv" => csv_writer::csv_writer(&data, output),
        _ => panic!("ERROR: Output extension {output_ext} is not supported"),
    };
    println!(
        "Finished converting {input} -> {output} in {:?}",
        now.elapsed()
    );
}
