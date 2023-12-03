use std::{fs::File, io::{BufRead, BufReader}, collections::HashMap};

use clap::Parser;

use nom::{
    bytes::complete::tag,
    character::complete::{u64, alphanumeric0},
    combinator::{map, all_consuming},
    multi::separated_list0,
    sequence::{delimited, tuple, separated_pair},
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

        println!("R: {} G: {} B: {}", min_r, min_g, min_b);

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
            Err(_) => 0
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
        let parsed = all_consuming(line_parser)(&range);
        answer += match parsed {
            Ok(v) => if v.1.valid(&limits) { v.1.id } else { 0 },
            Err(_) => panic!("Something couldn't get parsed correctly: {}", range)
        }
    }
    
    println!("Answer: {}", answer);
}

/// Start of the nom parser for each Game / line of the input file
fn line_parser(s: &str) -> IResult<&str, Game> {
    map(
        tuple((
            // Gets the Game ID
            delimited(tag("Game "), u64, tag(": ")),
            // Gets the Vec<Grab> from everything to the right of the 'Game 1: '
            separated_list0(tag("; "), parse_grab)
        )),
        |(id, grabs)| {
            println!("grabs {:?}", grabs);
            Game { id, grabs }
        }
    )(s)
}

/// Parses a Grab, as in '1 red, 2 blue, 3 green'
fn parse_grab(i: &str) -> IResult<&str, Grab> {
    map(
        separated_list0(tag(", "), parse_grab_entry),
        |grabs| Grab { grabs }
    )(i)
} 

/// Parses a GrabEntry, as in '1 red'
fn parse_grab_entry(i: &str) -> IResult<&str, GrabEntry> {
    map(
        separated_pair(u64, tag(" "), alphanumeric0),
        |(count, color)| { GrabEntry {count, color: String::from(color)} }
    )(i)
}