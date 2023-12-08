use std::{fs::File, io::{BufRead, BufReader}, collections::HashMap};

use clap::Parser;

use nom::{
    bytes::complete::tag,
    character::complete::{u64, alphanumeric1, multispace1},
    combinator::{map, all_consuming},
    multi::separated_list1,
    sequence::{delimited, tuple, separated_pair},
    IResult,
};

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

fn branch_parser(s: &str) -> IResult<&str, (&str, &str, &str)> {
    map(
        tuple((
            alphanumeric1,
            tag(" = ("),
            alphanumeric1,
            tag(", "),
            alphanumeric1,
            tag(")")
        )),
        |(key, _, left, _, right, _)| {
            (key, left, right)
        }
    )(s)
}

fn gcd(mut x: u64, mut y: u64) -> u64 {
    while x != y {
        if x > y {
            x = x - y;
        } else {
            y = y - x;
        }
    }
    x
}

fn lcm(x: u64, y: u64) -> u64 {
    x * y / gcd(x, y)
}

fn main() -> Result<(), ParseError> {
    let args = Args::parse();
    
    let input_file = File::open(args.input).unwrap();
    let input_ranges = parse(BufReader::new(input_file))?;
    
    let steps = input_ranges.get(0).unwrap();
    
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut currents = vec![];
    
    for range in input_ranges[2..].iter() {
        match all_consuming(branch_parser)(range) {
            Ok(p) => {
                map.insert(p.1.0, (p.1.1, p.1.2));
                if p.1.0.ends_with('A') {
                    currents.push(p.1.0);
                }
            },
            Err(e) => panic!("Parsing error! {}", e)
        }
    }
    
    println!("Starting points: {}", currents.len());
    
    let mut total_dist = 1;
    for one in &currents {
        let mut current = *one;
        for (step, d) in steps.bytes().cycle().enumerate() {
            let next = map.get(current).unwrap();
            match d {
                b'L' => current = next.0,
                b'R' => current = next.1,
                _ => panic!("Unknown step!")
            };
            if current.ends_with('Z') {
                total_dist = lcm(total_dist, u64::try_from(step+1).unwrap());
                break
            }
        }        
    }
    println!("Steps: {}", total_dist);    
    Ok(())
}    
