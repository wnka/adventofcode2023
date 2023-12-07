use std::{fs::File, io::{BufRead, BufReader, Read}, str, ops::Range};

use clap::Parser;

use nom::{
    bytes::complete::{tag, is_not, take_until},
    character::complete::{u64, multispace0, multispace1, alphanumeric1, space1, one_of},
    combinator::{map, all_consuming, map_res},
    multi::{separated_list0, separated_list1, many1},
    sequence::{delimited, tuple, preceded, terminated, pair},
    IResult, branch::alt,
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

#[derive(Debug)]
struct Block {
    from: String,
    to: String,
    lines: Vec<Vec<u64>>
}

struct Ranges {
    source: Range<u64>,
    destination: Range<u64>
}

fn chunk_parser(s: &str) -> IResult<&str, Block> {
    map(
        tuple((
        alphanumeric1,
        tag("-to-"),
        alphanumeric1,
        tag(" map:\n"),
        //separated_list0(tag("\n"), test)
        //separated_list0(tag("\n"), test)
        separated_list1(tag("\n"),
        separated_list1(tag(" "),u64))
        ))
    , |(a, _, b, _, c)| {
        println!("{:?}", c);
        Block { from: String::from(a), to: String::from(b), lines: c}
    })(s)
}

/*
fn parse_ranges(s: &str) -> IResult<&str, Ranges> {
    
}
*/

fn parse<T>(input_buffer: T) -> Result<Vec<String>, ParseError> where T: BufRead {
    let mut reader = BufReader::new(input_buffer);
    let mut block = vec![];
    reader.read_to_end(&mut block).unwrap();
    let file_contents = str::from_utf8(&block).unwrap();
    let blocks = file_contents.split("\n\n");
    for block in blocks {
        println!("BLOCK: \"{}\" END BLOCK", block);
        println!("{:?}", chunk_parser(&block));
    }
    Ok(Vec::new())
}

fn main() -> Result<(), ParseError> {
    let args = Args::parse();

    let input_file = File::open(args.input).unwrap();
    let input_ranges = parse(BufReader::new(input_file))?;

    for range in input_ranges {
        println!("{}", range);
    }

    let test = "seed-to-soil map:";
    /*
    map(
        tuple((
            is_not("-"),
            tag("to-"),
            delimited(alphanumeric1, multispace1, tag("map:"))
        )),
        |(a,b,c)| { println!("a {}, b {}", a,b)}
    )(test);
    */

    Ok(())
}
