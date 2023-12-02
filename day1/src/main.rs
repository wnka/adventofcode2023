use std::{fs::File, io::{BufRead, BufReader}};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename to read
    #[arg(short, long)]
    input: String,
}

#[derive(Debug)]
enum ParseError {
    Error
}

fn parse<T>(input_buffer: T) -> Result<Vec<String>, ParseError> where T: BufRead {
    let mut ranges = Vec::new();
    let lines = BufReader::new(input_buffer).lines();
    for line in lines {
        match line {
            Ok(s) => ranges.push(s),
            Err(_) => return Err(ParseError::Error)
        }
    }
    Ok(ranges)
}

fn main() -> Result<(), ParseError> {
    let args = Args::parse();

    let input_file = File::open(args.input).unwrap();
    let input_ranges = parse(BufReader::new(input_file))?;

    let mut sum = 0;

    for range in input_ranges {
        let digits : Vec<char> = range.chars().filter(|c| c.is_ascii_digit()).collect();
        let value = digits.first().unwrap().to_digit(10).unwrap() * 10 + digits.last().unwrap().to_digit(10).unwrap();
        sum += value;
        println!("Input: {}, Val {}, sum: {}", range, value, sum);
    }   

    println!("Answer: {}", sum);    

    Ok(())
}
