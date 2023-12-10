use std::{fs::File, io::{BufRead, BufReader}};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// Command line args
struct Args {
    /// Filename to read
    #[arg(short, long)]
    input: String,

    /// Which part of the day we're solving
    /// Usually only 1 or 2
    /// Defaults to 1
    #[arg(short, long, default_value_t = 1)]
    part: u8,
}

#[derive(Debug)]
enum ParseError {
    Error
}

/// Read an input file and return a Ok(Vec<String>) with one String per line
/// If something weird happens, return Err(ParseError::Error)
fn parse<T>(input_buffer: T) -> Result<Vec<String>, ParseError> where T: BufRead {
    let mut result = Vec::new();
    let lines = BufReader::new(input_buffer).lines();
    for line in lines {
        match line {
            Ok(s) => result.push(s),
            Err(_) => return Err(ParseError::Error)
        }
    }
    Ok(result)
}

fn main() -> Result<(), ParseError> {
    let args = Args::parse();

    let input_file = File::open(args.input).unwrap();
    let input_ranges = parse(BufReader::new(input_file))?;

    let mut input_lines : Vec<Vec<i64>> = vec![];

    for range in input_ranges {
        input_lines.push(range.split(' ').map(|v| v.parse().unwrap()).collect());
    }

    println!("Answer: {}", input_lines.into_iter().map(|v| get_next_element(v)).sum::<i64>());

    Ok(())
}

fn get_next_element(input: Vec<i64>) -> i64 {
    let mut dx : Vec<Vec<i64>> = vec![input];
    let mut index = 0;
    while let Some(array) = dx.get(index) {
        if array.iter().all(|v| *v == 0) {
            break;
        }
        dx.push(array.iter().zip(array[1..].iter()).into_iter().map(|(v, nextv)| nextv - v).collect());
        index += 1;
    }

    println!("dx: {:?}", dx);
    dx.iter().map(|v| v.last().unwrap()).sum()
}