use std::{fs::File, io::{BufRead, BufReader}, ops::Range, cmp::{max, min}};

use clap::Parser;

use regex::Regex;

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
    
    /// Debug output
    #[arg(short, long)]
    debug: bool,
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

fn intersect(a: &Range<usize>, b: &Range<usize>) -> bool {
    // Need to subtract off 1 from end since Ranges are
    // start <= x < end
    max(a.start, b.start) <= min(a.end-1, b.end-1)
}

#[derive(Debug, Clone)]
struct CandidatePart {
    value: usize,
    range: Range<usize>,
    row: usize,
    is_part: bool,
}

#[derive(Debug, Clone)]
struct Symbol {
    value: String,
    range: Range<usize>,
    row: usize,
    adjecent_parts: Vec<CandidatePart>,
}

fn main() -> Result<(), ParseError> {
    let args = Args::parse();
    
    let input_file = File::open(args.input).unwrap();
    let input_ranges = parse(BufReader::new(input_file))?;
    
    match args.part {
        1 => part_one(input_ranges, args.debug),
        2 => part_two(input_ranges, args.debug),
        _ => panic!("unknown part")
    }
    
    Ok(())
}

// correct value for my input was 550934
fn part_one(input_ranges: Vec<String>, debug: bool) {
    let mut rows: Vec<Vec<CandidatePart>> = Vec::new();
    for _ in 0..input_ranges.len() {
        rows.push(Vec::new());
    }
    
    let mut symbols: Vec<Vec<Symbol>> = Vec::new();
    for _ in 0..input_ranges.len() {
        symbols.push(Vec::new());
    }
    for (row, line) in input_ranges.iter().enumerate() {
        if debug {
            println!("ROW {}: {}", row, line);
        }
        let numbers_re = Regex::new("[0-9]+").unwrap();
        for extract in numbers_re.captures_iter(&line).map(|e| e.get(0).unwrap()) {
            rows.get_mut(row).unwrap().push(CandidatePart { value: extract.as_str().parse().unwrap(), range: extract.range(), row, is_part: false});
        }
        
        let symbols_re = Regex::new("[^0-9\\.]").unwrap();
        for extract in symbols_re.captures_iter(&line).map(|e| e.get(0).unwrap()) {
            // Symbol range, expand it to be 3 units wide to catch diagonals
            let expand_range = max(0, extract.range().start-1)..min(line.len()-1, extract.range().end+1);
            let symbol = Symbol { value: extract.as_str().to_string(), range: expand_range, row, adjecent_parts: Vec::new() };
            symbols.get_mut(row).unwrap().push(symbol.clone());
            
            // Add the symbols to the rows above and below to make things easier.
            if row > 0 {
                symbols.get_mut(row-1).unwrap().push(symbol.clone());
            }
            if row < symbols.len()-1 {
                symbols.get_mut(row+1).unwrap().push(symbol.clone());
            }
        }
    }
    
    let mut running_sum = 0;
    for (row, i) in rows.iter_mut().enumerate() {
        for candidate in i.iter_mut() {
            for symbol in symbols.get(row).unwrap().iter() {
                if intersect(&symbol.range, &candidate.range) {
                    running_sum += candidate.value;
                    break;
                }
            }
        }
        if debug {
            println!("row {}: {:?}", row, i);
        }
    }
    
    if debug {
        for (row, i) in symbols.iter().enumerate() {
            println!("row {}: {:?}", row, i);
        }
    }
    
    println!("Answer: {}", running_sum);
}

fn part_two(input_ranges: Vec<String>, debug: bool) {
    let mut rows: Vec<Vec<CandidatePart>> = Vec::new();
    for _ in 0..input_ranges.len() {
        rows.push(Vec::new());
    }
    
    let mut symbols: Vec<Vec<Symbol>> = Vec::new();
    for _ in 0..input_ranges.len() {
        symbols.push(Vec::new());
    }
    for (row, line) in input_ranges.iter().enumerate() {
        if debug {
            println!("ROW {}: {}", row, line);
        }
        let numbers_re = Regex::new("[0-9]+").unwrap();
        for extract in numbers_re.captures_iter(&line).map(|e| e.get(0).unwrap()) {
            rows.get_mut(row).unwrap().push(CandidatePart { value: extract.as_str().parse().unwrap(), range: extract.range(), row, is_part: false});
        }
        
        let symbols_re = Regex::new("[\\*]").unwrap();
        for extract in symbols_re.captures_iter(&line).map(|e| e.get(0).unwrap()) {
            // Symbol range, expand it to be 3 units wide to catch diagonals
            let expand_range = max(0, extract.range().start-1)..min(line.len()-1, extract.range().end+1);
            let symbol = Symbol { value: extract.as_str().to_string(), range: expand_range, row, adjecent_parts: Vec::new() };
            symbols.get_mut(row).unwrap().push(symbol.clone());
        }
    }
    
    let mut running_sum = 0;
    for (row, i) in symbols.iter().enumerate() {
        for symbol in i.iter() {
            let mut adjecents : Vec<CandidatePart> = Vec::new();
            for row_idx in max(0, row-1)..min(rows.len(), row+2) {
                println!("row: {} row_idx: {}", row, row_idx);
                for candidate in rows.get(row_idx).unwrap() {
                    if intersect(&symbol.range, &candidate.range) {
                        adjecents.push(candidate.clone());
                    }
                }
            }
            if adjecents.len() == 2 {
                running_sum += adjecents.iter().fold(1, |acc, x| acc * x.value);
            }            
        }
    }
        
    println!("Answer: {}", running_sum);
}

#[cfg(test)]
mod tests {
    use crate::intersect;
    
    #[test]
    fn test_intersect() {
        let a = 0..2 as usize;
        let b = 2..4 as usize;
        assert!(!intersect(&a, &b));
        
        let a = 0..2 as usize;
        let b = 0..1 as usize;
        assert!(intersect(&a, &b));
        
        let a = 2..4 as usize;
        let b = 3..5 as usize;
        assert!(intersect(&a, &b));
        
    }
}