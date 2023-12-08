use std::{fs::File, io::{BufRead, BufReader}};

use nom::{
    bytes::complete::tag,
    character::complete::{u64, alphanumeric1, multispace1},
    combinator::{map, all_consuming},
    multi::separated_list1,
    sequence::{delimited, tuple, separated_pair},
    IResult,
};

use clap::Parser;

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

fn time_parser(s: &str) -> IResult<&str, Vec<u64>> {
    map(
        tuple((
            tag("Time:"), 
            multispace1, 
            separated_list1(multispace1, u64)
        )),
        |(_, _, vals)| vals )(s)
}

fn distance_parser(s: &str) -> IResult<&str, Vec<u64>> {
    map(
        tuple((
            tag("Distance:"), 
            multispace1, 
            separated_list1(multispace1, u64)
        )),
        |(_, _, vals)| vals )(s)
}

fn find_record_breaking_count(time: u64, record: u64) -> u64 {
    let mut count = 0;
    println!("Computing count for time: {} record: {}", time, record);
    for i in 1..=time {
        let time_charge = i;
        let time_move = time-i;
        let distance = time_move * time_charge;
        //println!("\tCharge: {}, Distance: {}", time_charge, distance);
        if distance > record { count += 1 }
    }
    count
}

fn main() -> Result<(), ParseError> {
    let args = Args::parse();

    let input_file = File::open(args.input).unwrap();
    let input_ranges = parse(BufReader::new(input_file))?;

    let time_vals = match all_consuming(time_parser)(input_ranges.get(0).unwrap())
    {
        Ok((_, v)) => v,
        Err(e) => panic!("Time vals parsing: {}", e)
    };

    println!("time: {:?}", time_vals);

    let distance_vals = match all_consuming(distance_parser)(input_ranges.get(1).unwrap())
    {
        Ok((_, v)) => v,
        Err(e) => panic!("Distance vals parsing: {}", e)
    };

    println!("distance: {:?}", distance_vals);

    assert_eq!(time_vals.len(), distance_vals.len());

    let mut answer = 1;
    for (time, distance) in time_vals.iter().zip(distance_vals.iter()) {
        let count = find_record_breaking_count(*time, *distance);
        println!("Time: {}, Distance: {}, Records: {}", time, distance, count);
        if count > 0 { answer *= count }
    }

    println!("Answer: {}", answer);

    Ok(())
}
