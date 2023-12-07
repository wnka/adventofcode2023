use std::{fs::File, io::{BufRead, BufReader, Read}, str, ops::Range, collections::{VecDeque, HashMap}};

use clap::Parser;

use nom::{
    bytes::complete::tag,
    character::complete::{u64, alphanumeric1},
    combinator::{map, all_consuming},
    multi::separated_list1,
    sequence::tuple,
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

#[derive(Debug, Clone)]
struct Block {
    from: String,
    to: String,
    ranges: Vec<Ranges>
}

impl Block {
    fn get_dest(&self, src: u64) -> u64 {
        for range in &self.ranges {
            if let Some(dest) = range.get_dest(src) {
                return dest;
            }
        }

        src
    }
}

#[derive(Debug, Clone)]
struct Ranges {
    source: Range<u64>,
    destination: Range<u64>
}

impl Ranges {
    fn get_dest(&self, src: u64) -> Option<u64> {
        match self.source.contains(&src) {
            true => {
                let offset = src - self.source.start;
                Some(self.destination.start + offset)
            },
            false => None
        }
    }
}

fn chunk_parser(s: &str) -> IResult<&str, Block> {
    map(
        tuple((
        alphanumeric1,
        tag("-to-"),
        alphanumeric1,
        tag(" map:\n"),
        separated_list1(tag("\n"),
        parse_ranges)
        ))
    , |(a, _, b, _, c)| {
        Block { from: String::from(a), to: String::from(b), ranges: c}
    })(s)
}

fn parse_ranges(s: &str) -> IResult<&str, Ranges> {
    map(
    separated_list1(tag(" "), u64),
    |v| {
        assert_eq!(v.len(), 3);
        let s_start = *v.get(1).unwrap();
        let d_start = *v.get(0).unwrap();
        let run = v.get(2).unwrap();
        Ranges { source: s_start..(s_start+run), destination: d_start..(d_start+run) }
    }
    )(s)
}

fn parse_seeds(s: &str) -> IResult<&str, Vec<u64>> {
    map(
        tuple((
            tag("seeds: "),
            separated_list1(tag(" "), u64)
        )),
        |(_, v)| { v }
    )(s)
}
fn parse<T>(input_buffer: T) -> Result<(Vec<u64>, Vec<Block>), ParseError> where T: BufRead {
    let mut reader = BufReader::new(input_buffer);
    let mut block = vec![];
    reader.read_to_end(&mut block).unwrap();
    let file_contents = str::from_utf8(&block).unwrap();
    let mut blocks : VecDeque<&str> = file_contents.split("\n\n").collect();
    let seeds: Vec<u64> = match all_consuming(parse_seeds)(blocks.pop_front().unwrap()) {
        Ok(s) => s.1,
        Err(e) => panic!("Seed Parser Problem! {:?}", e)
    };
    let blocks = blocks.into_iter().map(|b| {
        match chunk_parser(b) {
            Ok(s) => s.1,
            Err(e) => panic!("Parser problem! {:?}", e)
        }
    }).collect();
    Ok((seeds, blocks))
}

fn main() -> Result<(), ParseError> {
    let args = Args::parse();

    let input_file = File::open(args.input).unwrap();
    let (seeds, blocks) = parse(BufReader::new(input_file))?;

    println!("seeds: {:?}", seeds);

    let mut src2dst : HashMap<String, Block> = HashMap::new();
    for block in blocks {
        println!("{:?}", block);
        src2dst.insert(block.from.clone(), block);
    }

    for i in seeds {
        println!("seed: {}, next: {}", i, src2dst.get("seed").unwrap().get_dest(i));
    }

    Ok(())
}
