use std::{fs::File, io::{BufRead, BufReader}, collections::{HashSet, HashMap}, f32::consts::E};

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

#[derive(Debug, Clone)]
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

    fn compute_copies(&self) -> Vec<u64> {
        let winners : HashSet<u64> = self.winners.clone().into_iter().collect();
        let numbers : HashSet<u64> = self.numbers.clone().into_iter().collect();
        let matches : u32 = winners.intersection(&numbers).collect::<Vec<&u64>>().len().try_into().unwrap();
        let mut retval = Vec::new();
        for i in 1_u64..=matches.into() {
            retval.push(self.id + i);
        }
        retval
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
    let mut pt2_pending : Vec<u64> = Vec::new();
    let mut card_index: HashMap<u64, Card> = HashMap::new();
    for range in input_ranges {
        let card = match all_consuming(line_parser)(&range) {
            Ok(s) => s.1,
            Err(e) => panic!("Parser problem! {:?}", e)
        };
        card_index.insert(card.id, card.clone());

        answer += card.compute_score();

        pt2_pending.push(card.id);
    }

    let mut pt2_processed = Vec::new();
    loop {
        let pending = pt2_pending.pop();
        match pending {
            Some(p) => {
                pt2_processed.push(p);
                pt2_pending.append(&mut card_index.get(&p).unwrap().compute_copies())
            },
            None => break
        }
    }
    
    println!("Answer: {}", answer);
    println!("Pt2 Answer: {}", pt2_processed.len());

    Ok(())
}
