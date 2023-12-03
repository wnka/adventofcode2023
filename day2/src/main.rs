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

    fn values(&self) -> HashMap<&str, u64> {
        let mut vals = HashMap::new();
        for entry in &self.grabs {
            vals.insert(entry.color.as_str(), entry.count);
        }
        vals
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
    
    match args.part {
        1 => part_one(input_ranges),
        2 => part_two(input_ranges),
        _ => panic!("Unknown part")
    }

    Ok(())
}

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

fn part_one(input_ranges: Vec<String>) {
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