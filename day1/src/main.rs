use std::{fs::File, io::{BufRead, BufReader}};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename to read
    #[arg(short, long)]
    input: String,
}

#[derive(Debug)]
enum ParseError {
    Error
}

fn parse<T>(input_buffer: T) -> Result<Vec<String>, ParseError> where T: BufRead {
    let mut ranges = Vec::new();
    let lines = BufReader::new(input_buffer).lines();
    for line in lines {
        match line {
            Ok(s) => ranges.push(s),
            Err(_) => return Err(ParseError::Error)
        }
    }
    Ok(ranges)
}

fn main() -> Result<(), ParseError> {
    let args = Args::parse();
    
    let input_file = File::open(args.input).unwrap();
    let input_ranges = parse(BufReader::new(input_file))?;
    
    let mut sum = 0;
    
    for input in input_ranges {
        let range = words_to_numbers(input.clone());
        let digits_l : Vec<char> = range.0.chars().filter(|c| c.is_ascii_digit()).collect();
        let digits_r : Vec<char> = range.1.chars().filter(|c| c.is_ascii_digit()).collect();
        let value = digits_l.first().unwrap().to_digit(10).unwrap() * 10 + digits_r.last().unwrap().to_digit(10).unwrap();
        sum += value;
        println!("Input: {}, Val {}, sum: {}", input, value, sum);
    }   
    
    println!("Answer: {}", sum);    
    
    Ok(())
}

fn words_to_numbers(input: String) -> (String, String) {
    // you need to find the leftmost number and the right most number
    // for example: 'eightwo9three'
    // this should be 8wo93 -> 83
    // THE REAL TOUGH STUFF
    // also, twone -> 21 and oneight -> 18
    // Due to this, we need two copies of the string, one left most version and one right most version
    let mut wip_l = input.clone();
    let mut wip_r = input.clone();
    let number_words = [("one", "1"), 
    ("two", "2"), 
    ("three", "3"), 
    ("four", "4"),
    ("five", "5"),
    ("six", "6"), 
    ("seven", "7"),
    ("eight", "8"), 
    ("nine", "9")];
    
    let mut right_most : Option<(usize, (&str, &str))> = None;
    let mut left_most: Option<(usize, (&str, &str))> = None;
    for number in number_words {
        let find_num = input.find(number.0);
        if find_num.is_some() {
            let find_num = find_num.unwrap();
            match left_most {
                Some(_n) => if find_num < left_most.unwrap().0 { left_most = Some((find_num, number))},
                None => left_most = Some((find_num, number))
            }
        }
        
        let rfind_num = input.rfind(number.0);
        if rfind_num.is_some() {
            let rfind_num = rfind_num.unwrap();
            match right_most {
                Some(_n) => if rfind_num > right_most.unwrap().0 { right_most = Some((rfind_num, number))},
                None => right_most = Some((rfind_num, number))
            }
        }
    }
    
    println!("Left most = {:?}", left_most);
    println!("Right most = {:?}", right_most);
    
    if left_most.is_some() {
        wip_l = wip_l.replace(left_most.unwrap().1.0, left_most.unwrap().1.1);
    }
    if right_most.is_some() {
        wip_r = wip_r.replace(right_most.unwrap().1.0, right_most.unwrap().1.1);
    }
    
    // For twone: wip_l = 2ne, wip_r = tw1
    (wip_l, wip_r)
}
