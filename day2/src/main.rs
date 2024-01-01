use std::{fs::File, io::{BufRead, BufReader}, collections::HashMap};

use clap::Parser;
use nom::combinator::all_consuming;

use day2::ParseError;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename to read
    #[arg(short, long)]
    input: String,
    
    #[arg(short, long, default_value_t = 1)]
    part: u8,
}

fn main() -> Result<(), ParseError> {
    let args = Args::parse();
    
    let input_file = File::open(args.input).unwrap();
    let input_ranges = day2::parse(BufReader::new(input_file))?;
    
    match args.part {
        1 => part_one(input_ranges),
        2 => part_two(input_ranges),
        _ => panic!("Unknown part")
    }
    
    Ok(())
}

/// Solver for part 2 of the question
/// Return the num of all the 'powers' for each game.
fn part_two(input_ranges: Vec<String>) {
    let mut answer = 0;
    
    for range in input_ranges {
        let parsed = all_consuming(day2::line_parser)(&range);
        answer += match parsed {
            Ok(v) => v.1.power(),
            Err(e) => panic!("Parse error: {:?}", e)
        }
    }
    
    println!("Answer: {}", answer);
}

/// Solver for part 1 of the question
/// Return the sum of the game IDs that are valid given the color limits.
fn part_one(input_ranges: Vec<String>) {
    let mut limits : HashMap<&str, u64> = HashMap::new();
    limits.insert("green", 13);
    limits.insert("red", 12);
    limits.insert("blue", 14);
    
    let mut answer = 0;
    
    for range in input_ranges {
        // all_consuming makes sure everything in the line gets parsed and there are no leftovers.
        let parsed = all_consuming(day2::line_parser)(&range);
        answer += match parsed {
            Ok(v) => if v.1.valid(&limits) { v.1.id } else { 0 },
            Err(_) => panic!("Something couldn't get parsed correctly: {}", range)
        }
    }
    
    println!("Answer: {}", answer);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    
    use day2::line_parser;
    
    #[test]
    fn test_it_all() {
        let input = "   Game    3   :     1 green     , 7 red ; 1 green    ,     9 red\t\t  , 3 blue     ;      4 blue, 5     red";
        let result = line_parser(input);
        assert!(result.is_ok());
        let result = result.ok();
        assert!(result.is_some());
        
        let result = result.unwrap();
        assert_eq!(result.0, "");
        assert_eq!(result.1.id, 3);
        
        let mut limits : HashMap<&str, u64> = HashMap::new();
        limits.insert("green", 13);
        limits.insert("red", 9);
        limits.insert("blue", 14);
        
        assert!(result.1.valid(&limits));
        
        let mut limits : HashMap<&str, u64> = HashMap::new();
        limits.insert("green", 13);
        limits.insert("red", 8);
        limits.insert("blue", 14);
        
        assert!(!result.1.valid(&limits));
        
        assert_eq!(result.1.power(), 36);
    }
}