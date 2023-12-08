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

fn main() -> Result<(), ParseError> {
    let args = Args::parse();

    let input_file = File::open(args.input).unwrap();
    let input_ranges = parse(BufReader::new(input_file))?;

    let steps = input_ranges.get(0).unwrap().chars();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    for range in input_ranges[2..].iter() {
        match all_consuming(branch_parser)(range) {
            Ok(p) => {
                map.insert(p.1.0, (p.1.1, p.1.2));
            },
            Err(e) => panic!("Parsing error! {}", e)
        }
    }

    let mut current = "AAA";
    let mut step_count = 0;
    for i in steps.cycle() {
        let next = map.get(current).unwrap();
        match i {
            'L' => current = next.0,
            'R' => current = next.1,
            _ => panic!("Unknown step!")
        };
        step_count += 1;
        if current == "ZZZ" { break }
    }

    println!("Steps: {}", step_count);


//    let test = "LLR";
//    for i in test.chars().cycle() {
 //       println!("{}", i);
 //   }

    Ok(())
}
