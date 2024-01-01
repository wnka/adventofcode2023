use std::{collections::HashMap, io::{BufRead, BufReader}};

use nom::{
    bytes::complete::tag,
    character::complete::{u64, alphanumeric0, multispace0},
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, tuple, separated_pair, preceded, terminated},
    IResult,
};

#[derive(Debug)]
/// A Game maps to a line in the input file
pub struct Game {
    pub id: u64,
    pub grabs: Vec<Grab>
}

impl Game {
    /// Given the 'limits', is this game valid?
    /// It's valid iff every individual Grab is valid.
    pub fn valid(&self, limits: &HashMap<&str, u64>) -> bool {
        self.grabs.iter().all(|val| val.valid(limits))
    }
    
    /// Find the 'power'
    /// Look across all grabs and find the number of cubes
    /// required for each color to make all the grabs possible.
    /// Basically, find the max of each color and multiply them together.
    /// If a color doesn't show up across all grabs, the result is 0.
    pub fn power(&self) -> u64 {
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
pub struct GrabEntry {
    count: u64,
    color: String,
}

impl GrabEntry {
    /// Is this entry valid given the passed in 'limits'
    /// If the limit for this color is 5 and our value is 2, it's valid.
    /// If the limit for this color is 5 and our value is 6, it's not valid.
    pub fn valid(&self, limits: &HashMap<&str, u64>) -> bool {
        match limits.get(self.color.as_str()) {
            Some(limit) => self.count <= *limit,
            None => panic!("No limit for that color!")
        }
    }
}

#[derive(Debug)]
/// Represents the cubes and colors pulled out in a grab of cubes
/// i.e. 2 red, 3 green, 1 blue
pub struct Grab {
    grabs: Vec<GrabEntry>
}

impl Grab {
    /// The grab is valid iff each color is under the limit.
    pub fn valid(&self, limits: &HashMap<&str, u64>) -> bool {
        self.grabs.iter().all(|val| val.valid(limits))
    }
    
    /// Return a hash map of color -> count for this grab.
    pub fn values(&self) -> HashMap<&str, u64> {
        let mut vals = HashMap::new();
        for entry in &self.grabs {
            vals.insert(entry.color.as_str(), entry.count);
        }
        vals
    }
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
pub fn line_parser(s: &str) -> IResult<&str, Game> {
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

#[derive(Debug)]
pub enum ParseError {
    Error
}

/// Read an input file and return a Ok(Vec<String>) with one String per line
/// If something weird happens, return Err(ParseError::Error)
pub fn parse<T>(input_buffer: T) -> Result<Vec<String>, ParseError> where T: BufRead {
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

#[derive(Default)]
pub struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

pub struct Game2 {
    rounds: Vec<Round>,
}

pub fn generate(input: &str) -> Vec<Game2> {
    input
        .lines()
        .map(|line| Game2 {
            rounds: line
                .split(": ")
                .skip(1)
                .next()
                .unwrap()
                .split("; ")
                .map(|round| {
                    let mut res = Round::default();
                    for color in round.split(", ") {
                        let mut it = color.split(" ");
                        let num = u32::from_str_radix(it.next().unwrap(), 10).unwrap();
                        match it.next().unwrap() {
                            "red" => res.red = num,
                            "blue" => res.blue = num,
                            "green" => res.green = num,
                            _ => panic!(),
                        }
                    }
                    res
                })
                .collect(),
        })
        .collect()
}