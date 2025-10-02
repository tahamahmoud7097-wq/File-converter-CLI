mod utils;
use utils::*;

fn main() {
    let args: Vec<String> =
    std::env::args().collect();
    if args.len() != 3 {
        panic!("Invalid usage, correct usage: prog <input> <output>");
    }
    let input = &args[1];
    let output = &args[2];
    let input_ext = input.split(".").last().unwrap_or("");
    let output_ext = output.split(".").last().unwrap_or("");
    let now = std::time::Instant::now();
    let data: UniversalData =
    match input_ext {
        "txt" => txt_reader::read_from_txt(input, output_ext),
        "json" => json_reader::json_reader(input),
        "toml" => toml_reader::toml_reader(input),
        "csv" => csv_reader::csv_reader(input),
        _ => panic!("Input extension {input_ext} is not supported"),
    };
    match output_ext {
        "json" => json_writer::json_writer(&data, output),
        "toml" => toml_writer::toml_writer(&data, output),
        "csv" => csv_writer::csv_writer(&data, output),
        _ => panic!("Output extension {output_ext} is not supported"),
    };
    println!("Finished converting {input} -> {output} in {:?}", now.elapsed());
}


