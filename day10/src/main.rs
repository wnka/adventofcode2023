use std::{fs::File, io::{BufRead, BufReader}, fmt::{Error, self}, path::Display};

use clap::Parser;

use colored::*;

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

enum Move {
    North,
    South,
    East,
    West,
}

fn go(direction: &Move, x: usize, y: usize) -> (usize, usize) {
    match direction {
        Move::North => (x, y-1),
        Move::South => (x, y+1),
        Move::East => (x+1, y),
        Move::West => (x-1, y)
    }
}

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
#[derive(Debug)]
enum Direction {
    NorthSouth, // |
    EastWest, // -
    NorthEast, // L
    NorthWest, // J
    SouthWest, // 7
    SouthEast, // F
    Ground, // .
    Starting
}

impl Direction {
    fn next_move(&self, incoming_step: &Move) -> Option<Move> {
        match (self, incoming_step) {
            (Direction::NorthSouth, Move::North) => Some(Move::North), // I came in through the north, I should go north
            (Direction::NorthSouth, Move::South) => Some(Move::South),
            (Direction::EastWest, Move::West) => Some(Move::West),
            (Direction::EastWest, Move::East) => Some(Move::East),
            (Direction::NorthEast, Move::West) => Some(Move::North),
            (Direction::NorthEast, Move::South) => Some(Move::East),
            (Direction::NorthWest, Move::East) => Some(Move::North),
            (Direction::NorthWest, Move::South) => Some(Move::West),
            (Direction::SouthEast, Move::West) => Some(Move::South),
            (Direction::SouthEast, Move::North) => Some(Move::East),
            (Direction::SouthWest, Move::East) => Some(Move::South),
            (Direction::SouthWest, Move::North) => Some(Move::West),
            (_,_) => None
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Self::NorthSouth => '|',
            Self::EastWest => '-',
            Self::NorthEast => 'L',
            Self::NorthWest => 'J',
            Self::SouthWest => '7',
            Self::SouthEast => 'F',
            Self::Ground => '.',
            Self::Starting => 'S',
        };
        write!(f, "{}", char)
    }
}

struct Point {
    x: usize,
    y: usize,
    direction: Direction,
    color: Option<colored::Color>,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Some(c) => write!(f, "{}", ColoredString::from(format!("{}", self.direction)).color(c)),
            None => write!(f, "{}", self.direction)
        }
    }
}

impl std::convert::TryFrom<char> for Direction {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let ev = match value {
            '|' => Self::NorthSouth,
            '-' => Self::EastWest,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            '.' => Self::Ground,
            'S' => Self::Starting,
            _ => panic!("Unknown direction!")
        };
        Ok(ev)
    }
}

fn main() -> Result<(), ParseError> {
    let args = Args::parse();

    let input_file = File::open(args.input).unwrap();
    let input_ranges = parse(BufReader::new(input_file))?;
    let mut map : Vec<Vec<Point>> = vec![];
    let mut cur_x = None;
    let mut cur_y = None;
    for (row, range) in input_ranges.iter().enumerate() {
        let mut parsed_row = vec![];
        for (col, val) in range.chars().enumerate()
        {
            let dir = Direction::try_from(val).unwrap();
            let color = match dir {
                Direction::Starting => {
                    cur_x = Some(col);
                    cur_y = Some(row);
                    Some(Color::Green)
                },
                _ => None
            };
            parsed_row.push(Point {x: col, y: row, direction: dir, color});
        }
        map.push(parsed_row);
    }

    assert!(cur_x.is_some());
    assert!(cur_y.is_some());

    let mut cur_x = cur_x.unwrap();
    let mut cur_y = cur_y.unwrap();

    println!("Starting x: {} y: {}", cur_x, cur_y);

    // try left
    //let to_the_left = map.get_mut(cur_x+1).unwrap();

    let mut step_count = 0;
    // TODO this is a cheat based on me looking at the input
    let mut step = Some(Move::North);
    while let Some(s) = step {
        step_count += 1;
        (cur_x, cur_y) = go(&s, cur_x, cur_y);
        let p = map.get_mut(cur_y).unwrap().get_mut(cur_x).unwrap();
        p.color = match p.color {
            None => Some(Color::Red),
            Some(s) => Some(s)
        };
        step = p.direction.next_move(&s);

        if let Direction::Starting = p.direction {
            break
        }
    }

    for row in map.iter_mut() {
        for col in row.iter_mut() {
            //col.color = Some(Color::Red);
            print!("{}", col);
        }
        println!();
    }

    println!("Step count: {}, farthest: {}", step_count, step_count/2);

    // TODO: For part two, I can color the stuff not in a loop. I can start at
    // an edge, and if that point either touches something blue
    // (up/down/left/right) or touches an edge (the direction goes out of
    // bounds) it's blue. Then I just count everything that's not blue and not
    // red.

    Ok(())
}
