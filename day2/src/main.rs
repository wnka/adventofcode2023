use std::{fs::File, io::{BufRead, BufReader}, collections::HashMap};

use clap::Parser;

use nom::{
    bytes::complete::tag,
    character::complete::{u64, alphanumeric0, multispace0},
    combinator::{map, all_consuming},
    multi::separated_list0,
    sequence::{delimited, tuple, separated_pair, preceded, terminated},
    IResult,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename to read
    #[arg(short, long)]
    input: String,
    
    #[arg(short, long, default_value_t = 1)]
    part: u8,
}

#[derive(Debug)]
enum ParseError {
    Error
}

#[derive(Debug)]
/// A Game maps to a line in the input file
struct Game {
    id: u64,
    grabs: Vec<Grab>
}

impl Game {
    /// Given the 'limits', is this game valid?
    /// It's valid iff every individual Grab is valid.
    fn valid(&self, limits: &HashMap<&str, u64>) -> bool {
        self.grabs.iter().all(|val| val.valid(limits))
    }
    
    /// Find the 'power'
    /// Look across all grabs and find the number of cubes
    /// required for each color to make all the grabs possible.
    /// Basically, find the max of each color and multiply them together.
    /// If a color doesn't show up across all grabs, the result is 0.
    fn power(&self) -> u64 {
        let mut greens = Vec::new();
        let mut blues = Vec::new();
        let mut reds = Vec::new();
        for grab in &self.grabs {
            for (k, v) in grab.values() {
                match k {
                    "red" => reds.push(v),
                    "green" => greens.push(v),
                    "blue" => blues.push(v),
                    _ => panic!("Unknown color!"),
                }
            }
        }
        
        let min_r = reds.iter().max().unwrap_or(&0);
        let min_g = greens.iter().max().unwrap_or(&0);
        let min_b = blues.iter().max().unwrap_or(&0);
        
        min_r * min_g * min_b
    }
}

#[derive(Debug)]
/// Represents one color in a grab
/// i.e. 2 green
struct GrabEntry {
    count: u64,
    color: String,
}

impl GrabEntry {
    /// Is this entry valid given the passed in 'limits'
    /// If the limit for this color is 5 and our value is 2, it's valid.
    /// If the limit for this color is 5 and our value is 6, it's not valid.
    fn valid(&self, limits: &HashMap<&str, u64>) -> bool {
        match limits.get(self.color.as_str()) {
            Some(limit) => self.count <= *limit,
            None => panic!("No limit for that color!")
        }
    }
}

#[derive(Debug)]
/// Represents the cubes and colors pulled out in a grab of cubes
/// i.e. 2 red, 3 green, 1 blue
struct Grab {
    grabs: Vec<GrabEntry>
}

impl Grab {
    /// The grab is valid iff each color is under the limit.
    fn valid(&self, limits: &HashMap<&str, u64>) -> bool {
        self.grabs.iter().all(|val| val.valid(limits))
    }
    
    /// Return a hash map of color -> count for this grab.
    fn values(&self) -> HashMap<&str, u64> {
        let mut vals = HashMap::new();
        for entry in &self.grabs {
            vals.insert(entry.color.as_str(), entry.count);
        }
        vals
    }
}

/// Read an input file and return a Ok(Vec<String>) with one String per line
/// If something weird happens, return Err(ParseError::Error)
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
        let parsed = all_consuming(line_parser)(&range);
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
        let parsed = all_consuming(line_parser)(&range);
        answer += match parsed {
            Ok(v) => if v.1.valid(&limits) { v.1.id } else { 0 },
            Err(_) => panic!("Something couldn't get parsed correctly: {}", range)
        }
    }
    
    println!("Answer: {}", answer);
}

/// Start of the nom parser for each Game / line of the input file
/// Example:
/// Game 1: 2 green, 12 blue; 6 red, 6 blue; 8 blue, 5 green, 5 red; 5 green, 13 blue; 3 green, 7 red, 10 blue; 13 blue, 8 red
///      ^ = Game id
/// Game 1: 2 green, 12 blue; 6 red, 6 blue; 8 blue, 5 green, 5 red; 5 green, 13 blue; 3 green, 7 red, 10 blue; 13 blue, 8 red
///         ^^^^^^^^^^^^^^^^ = Grab
/// Game 1: 2 green, 12 blue; 6 red, 6 blue; 8 blue, 5 green, 5 red; 5 green, 13 blue; 3 green, 7 red, 10 blue; 13 blue, 8 red
///                  ^^^^^^^ = GrabEntry
/// TODO: Doesn't handle whitespace variations.
fn line_parser(s: &str) -> IResult<&str, Game> {
    map(
        tuple((
            // Gets the Game ID
            delimited(delimited(multispace0, tag("Game"), multispace0), terminated(u64, multispace0), tag(":")),
            // Gets the Vec<Grab> from everything to the right of the 'Game 1: '
            separated_list0(delimited(multispace0, tag(";"), multispace0), parse_grab)
        )),
        |(id, grabs)| {
            Game { id, grabs }
        }
    )(s)
}

/// Parses a Grab, as in '1 red, 2 blue, 3 green'
fn parse_grab(i: &str) -> IResult<&str, Grab> {
    map(
        separated_list0(delimited(multispace0, tag(","), multispace0), parse_grab_entry),
        |grabs| Grab { grabs }
    )(i)
} 

/// Parses a GrabEntry, as in '1 red'
fn parse_grab_entry(i: &str) -> IResult<&str, GrabEntry> {
    map(
        separated_pair(preceded(multispace0, u64), multispace0, terminated(alphanumeric0, multispace0)),
        |(count, color)| { GrabEntry {count, color: String::from(color)} }
    )(i)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    
    use crate::line_parser;
    
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