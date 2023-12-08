use std::{fs::File, io::{BufRead, BufReader}, str::Chars, cmp::Ordering};
use std::collections::HashSet;

use clap::Parser;
use nom::{combinator::{all_consuming, map}, bytes::complete::{take_until, tag}, character::complete::{u64, alphanumeric1}, sequence::separated_pair, IResult};

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

#[derive(Debug, Ord, Eq)]
struct Game {
    input: String,
    hand: Vec<u8>,
    bid: u64,
    hand_type: u8 // 5 of a kind = 5, 4 of a kind = 4, etc
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let hand_cmp = self.hand_type.cmp(&other.hand_type);
        if hand_cmp != Ordering::Equal { 
            println!("CMP!");
            return Some(hand_cmp);
        }

        assert_eq!(self.hand.len(), other.hand.len());
        for (s, o) in self.hand.iter().zip(other.hand.iter()) {
            let card_cmp = s.cmp(o);
            match s.cmp(o) {
                Ordering::Equal => (),
                _ => { return Some(card_cmp); }
            }
        }

        Some(Ordering::Equal)
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.input == other.input &&
        self.bid == other.bid
    }
}

impl Game {
    fn new(input: &str, bid: u64) -> Self {
        let mut hand = vec![];
        for c in input.chars() {
            let num_val = match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                _ => c.to_digit(10).unwrap()
            };
            hand.push(num_val as u8);
        }

        let set: HashSet<char> = HashSet::from_iter(input.chars().collect::<Vec<_>>());
        let hand_type = set.iter().map(|v| input.matches(*v).count()).max().unwrap() as u8;

        //let hand_type = HashSet::from_iter(test.chars().collect::<Vec<_>>());
        Self { input: String::from(input), hand: hand, bid:bid, hand_type:hand_type }
    }
}

fn line_parser(s: &str) -> IResult<&str, Game> {
    map(
        separated_pair(alphanumeric1, tag(" "), u64),
        |(hand, bet)| {
            Game::new(hand, bet)
        })(s)
}


fn main() -> Result<(), ParseError> {
    let args = Args::parse();

    let input_file = File::open(args.input).unwrap();
    let input_ranges = parse(BufReader::new(input_file))?;

    let mut games = vec![];
    for range in input_ranges {
        println!("{}", range);
        match all_consuming(line_parser)(&range) {
            Ok(g) => games.push(g.1),
            Err(e) => panic!("Parse error! {}", e)
        }

    }
    games.sort();
    games.reverse();

    let mut answer = 0;
    for (index, game) in games.iter().enumerate() {
        println!("{}: {:?}", index, game);
        answer += (index+1) * game.bid as usize;
    }

    println!("Answer: {}", answer);

    Ok(())
}
