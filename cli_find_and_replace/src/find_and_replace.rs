use text_colorizer::*;
use std::env;
use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Arguments {
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String,
}

fn print_help() {
    eprintln!("{} - place a string with a new string", "Find and Replace".green());
    eprintln!("Useage: <target string> <replacement string> <INPUT FILE> <OUTPUT FILE>");
}

pub fn run(){
    let args = parse_args();
    read_write(&args);
}

fn parse_args() -> Arguments{
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 4
    {
        print_help();
        eprintln!("{} wrong number of arguments. Expected four, got {}", "Error".red().bold(), args.len());
        std::process::exit(1);
    }
    
    Arguments{
        pattern: args[0].clone(),
        replace: args[1].clone(),
        input_file: args[2].clone(),
        output_file: args[3].clone()
    }
}

fn read_write(args: &Arguments){
    let data = match fs::read_to_string(&args.input_file){
        Ok(f) => f,
        Err(e) => {
            eprintln!("{} failed to read from file {}: {:?}", "Error".red().bold(), args.input_file, e);
            std::process::exit(1)
        }
    };
    
    let replace_data = match replace(&args.pattern, &args.replace, &data){
        Ok(d) => d,
        Err(e) => {
            eprintln!("{} failed to replace text: {:?} {}", "Error".red().bold(), args.output_file, e);
            std::process::exit(1);
        }
    };
    
    match fs::write(&args.output_file, &replace_data) { 
        Ok(_) => {},
        Err(e) => {
            eprintln!("{} failed to write to file {}: {:?}", "Error".red().bold(), args.output_file, e);
            std::process::exit(1)
        }
    };
}

fn replace(target: &str, rep: &str, data: &str) -> Result<String, regex::Error>{
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(data, rep).to_string())
}