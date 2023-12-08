use std::{fs::File, io::{BufRead, BufReader}, str::Chars};
use std::collections::HashSet;

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

#[derive(Debug)]
struct Game {
    input: String,
    hand: Vec<u8>,
    bid: u64,
    hand_type: u8 // 5 of a kind = 5, 4 of a kind = 4, etc
}

impl Game {
    fn new(input: &str, bid: u64) -> Self {
        let mut hand = vec![];
        for c in input.chars() {
            let num_val = match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                _ => c.to_digit(10).unwrap()
            };
            hand.push(num_val as u8);
        }

        let set: HashSet<char> = HashSet::from_iter(input.chars().collect::<Vec<_>>());
        let hand_type = set.iter().map(|v| input.matches(*v).count()).max().unwrap() as u8;

        //let hand_type = HashSet::from_iter(test.chars().collect::<Vec<_>>());
        Self { input: String::from(input), hand: hand, bid:bid, hand_type:hand_type }
    }
}

fn main() -> Result<(), ParseError> {
    let args = Args::parse();

    let input_file = File::open(args.input).unwrap();
    let input_ranges = parse(BufReader::new(input_file))?;

    for range in input_ranges {
        println!("{}", range);
    }

    let test = "12345";

    let hand = Game::new(test, 123);
    println!("Hand: {:?}", hand);

    Ok(())
}
