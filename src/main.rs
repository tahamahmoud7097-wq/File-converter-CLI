mod utils;
use colored::Colorize;
use std::path::Path;
use std::process::exit;
use utils::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // Check if arguments are valid
    if args.len() != 3 {
        eprintln!(
            "{}",
            "ERROR: Invalid usage, correct usage: prog <input> <output>"
                .red()
                .bold()
        );
        exit(1);
    }
    let input: &String = &args[1];
    let output: &String = &args[2];
    // Check if input exists
    if !Path::new(input).exists() {
        eprintln!("{}", "ERROR: Input file doesn't exist.".red().bold());
        exit(1);
    }
    // Check if output exists, if it doesn't then create it
    if !Path::new(output).exists() {
        println!(
            "{}",
            format!(
                "Output file [{}] doesn't exist, creating a new file for it...",
                output
            )
            .truecolor(179, 245, 216)
            .italic()
        );
        std::fs::File::create(output).better_expect("ERROR: Couldn't create output file.");
    }
    let repo_link = "https://github.com/tahamahmoud7097-wq/File-converter-CLI"
        .truecolor(16, 101, 230)
        .bold();
    let input_ext: &str = input.split(".").last().unwrap_or("");
    let output_ext: &str = output.split(".").last().unwrap_or("");
    let now = std::time::Instant::now();
    let data: UniversalData = match input_ext {
        "txt" => txt_reader::read_from_txt(input, output_ext),
        "json" => json_reader::json_reader(input),
        "toml" => toml_reader::toml_reader(input),
        "csv" => csv_reader::csv_reader(input),
        _ => {
            eprintln!(
                "{} \n Open an issue at {}",
                format!(
                    "ERROR: Intput extension \"{}\" is not supported currently.",
                    input_ext
                )
                .red()
                .bold(),
                repo_link
            );
            exit(1);
        }
    };
    match output_ext {
        "json" => write_json::write_json(&data, output),
        "toml" => toml_writer::toml_writer(&data, output),
        "csv" => csv_writer::csv_writer(&data, output),
        _ => {
            eprintln!(
                "{} \n Open an issue at {}",
                format!(
                    "ERROR: Output extension \"{}\" is not supported currently.",
                    output_ext
                )
                .red()
                .bold(),
                repo_link
            );
            exit(1);
        }
    };
    println!(
        "Finished converting {} -> {} in {:?}",
        input.bright_green().bold(),
        output.bright_green().bold(),
        now.elapsed()
    );
}
