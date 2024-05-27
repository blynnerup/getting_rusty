use text_colorizer::*;

#[derive(Debug)]
struct Arguments {
    pattern: String,
    replacer: String,
    input_file: String,
    output_file: String,
}

fn main() {
    print_help();
}

fn print_help() {
    eprintln!("{} - place a string with a new string", "Find and Replace".green());
    eprintln!("Useage: <target string> <replacement string> <INPUT FILE> <OUTPUT FILE>");
}