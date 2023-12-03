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
struct Game {
    id: u64,
    grabs: Vec<Grab>
}

impl Game {
    fn valid(&self, limits: HashMap<&str, u64>) -> bool {
        self.grabs.iter().fold(true, |res, val| res && val.valid(limits.clone()))
    }
}

#[derive(Debug)]
struct GrabEntry {
    count: u64,
    color: String,
}

impl GrabEntry {
    fn valid(&self, limits: HashMap<&str, u64>) -> bool {
        match limits.get(self.color.as_str()) {
            Some(limit) => self.count <= *limit,
            None => panic!("No limit for that color!")
        }
    }
}

#[derive(Debug)]
struct Grab {
    grabs: Vec<GrabEntry>
}

impl Grab {
    fn valid(&self, limits: HashMap<&str, u64>) -> bool {
        self.grabs.iter().fold(true, |res, val| res && val.valid(limits.clone()))
    }
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
    
    let mut limits = HashMap::new();
    limits.insert("green", 13);
    limits.insert("red", 12);
    limits.insert("blue", 14);
    
    let mut answer = 0;

    for range in input_ranges {
        let parsed = all_consuming(line_parser)(&range);
        answer += match parsed {
            Ok(v) => if v.1.valid(limits.clone()) { v.1.id } else { 0 },
            Err(_) => 0
        }
    }
    
    println!("Answer: {}", answer);

    Ok(())
}

fn line_parser(s: &str) -> IResult<&str, Game> {
    map(
        tuple((
            delimited(tag("Game "), u64, tag(": ")),
            separated_list0(tag("; "), parse_grab)
        ))
        ,
        |(id, grabs)| {
            println!("grabs {:?}", grabs);
            Game { id, grabs }
        }
    )(s)
}

fn parse_grab(i: &str) -> IResult<&str, Grab> {
    map(
        separated_list0(tag(", "), parse_grab_entry),
        |grabs| Grab { grabs }
    )(i)
} 

fn parse_grab_entry(i: &str) -> IResult<&str, GrabEntry> {
    map(
        separated_pair(u64, tag(" "), alphanumeric0),
        |(count, color)| { GrabEntry {count: count, color: String::from(color)} }
    )(i)
}