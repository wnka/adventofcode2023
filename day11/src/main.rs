use std::{fs::File, io::{BufRead, BufReader}};

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

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn main() -> Result<(), ParseError> {
    let args = Args::parse();

    let input_file = File::open(args.input).unwrap();
    let input_ranges = parse(BufReader::new(input_file))?;

    let mut duped = vec![];

    let mut x_gaps = vec![];
    let mut y_gaps = vec![];


    for (i, range) in input_ranges.iter().enumerate() {
        duped.push(range.chars().collect::<Vec<char>>());
        if range.chars().all(|c| c == '.') {
            //duped.push(range.chars().collect::<Vec<char>>());
            y_gaps.push(i);
        }
        println!("{}", range);
    }

    let tpd = transpose(duped);

    let mut duped = vec![];
    for (i, range) in tpd.iter().enumerate() {
        duped.push(range.clone());
        if range.iter().all(|c| *c == '.') {
            //duped.push(range.clone());
            x_gaps.push(i);
        }
    }

    let input = transpose(duped);

    for line in &input {
        println!("{:?}", line);
    }

    println!("x-gaps: {:?}", x_gaps);
    println!("y-gaps: {:?}", y_gaps);

    let mut locations = vec![];

    for (y, line) in input.iter().enumerate() {
        for (x, val) in line.iter().enumerate() {
            if *val == '#' {
                println!("Found a galaxy at {},{}", x, y);
                locations.push((x,y));
            }
        }
    }

    let mut distances = 0;
    let source = locations.clone();
    for i in &source {
        for j in &source {
            let distance = ((i.0 - j.0) as i64).abs() + ((i.1 - j.1) as i64).abs();
            //println!("distance from {:?} to {:?}: {}", i, j, distance);
            distances += distance;
            let x_range = (std::cmp::min(i.0, j.0))..(std::cmp::max(i.0, j.0));
            let x_plus: i64 = x_gaps.iter().map(|g|{
                match x_range.contains(g) {
                    true => 999999,
                    false => 0
                }
            }).sum();

            let y_range = (std::cmp::min(i.1, j.1))..(std::cmp::max(i.1, j.1));
            let y_plus: i64 = y_gaps.iter().map(|g|{
                match y_range.contains(g) {
                    true => 999999,
                    false => 0
                }
            }).sum();

            distances += x_plus + y_plus;
        }
    }

    // Divide by 2 since I'm double counting
    println!("Distance sum: {}", distances/2);

    Ok(())
}
