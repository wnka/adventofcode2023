use std::{fs::File, io::{BufRead, BufReader}, collections::HashSet};

use clap::Parser;

use nom::{
    bytes::complete::tag,
    character::complete::{u64, multispace0, multispace1},
    combinator::{map, all_consuming},
    multi::separated_list0,
    sequence::{delimited, tuple, preceded, terminated},
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

#[derive(Debug)]
struct Card {
    id: u64,
    winners: Vec<u64>,
    numbers: Vec<u64>
}

impl Card {
    fn compute_score(&self) -> u32 {
        let winners : HashSet<u64> = self.winners.clone().into_iter().collect();
        let numbers : HashSet<u64> = self.numbers.clone().into_iter().collect();
        let matches : u32 = winners.intersection(&numbers).collect::<Vec<&u64>>().len().try_into().unwrap();
        if matches == 0 {
            0
        } else {
            2_u32.pow(matches-1)
        }
    }
}

fn line_parser(s: &str) -> IResult<&str, Card> {
    map(
        tuple((
            // Gets the Game ID
            delimited(delimited(multispace0, tag("Card"), multispace0), terminated(u64, multispace0), terminated(tag(":"), multispace0)),
            // Gets the Vec<Grab> from everything to the right of the 'Game 1: '
            separated_list0(delimited(multispace0, tag("|"), multispace0),
             separated_list0(multispace1, preceded(multispace0, u64)))
        )),
        |(id, winners)| {
            Card { 
                id, 
                winners: winners.get(0).unwrap().clone(), 
                numbers: winners.get(1).unwrap().clone()
            }
        }
    )(s)
}

fn main() -> Result<(), ParseError> {
    let args = Args::parse();
    
    let input_file = File::open(args.input).unwrap();
    let input_ranges = parse(BufReader::new(input_file))?;
    
    let mut answer = 0;
    for range in input_ranges {
        let card = match all_consuming(line_parser)(&range) {
            Ok(s) => s.1,
            Err(e) => panic!("Parser problem! {:?}", e)
        };

        println!("id {} score {}", card.id, card.compute_score());
        answer += card.compute_score();
    }
    
    println!("Answer: {}", answer);

    Ok(())
}
