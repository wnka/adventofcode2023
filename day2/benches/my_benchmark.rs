use std::{fs::File, io::BufReader};
use std::io::prelude::*;

use criterion::{criterion_group, criterion_main, Criterion};
use nom::combinator::all_consuming;
use nom::multi::many0;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input_file = File::open("input.txt").unwrap();
    let input_ranges = day2::parse(BufReader::new(input_file)).unwrap();

    let mut input_file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    input_file.read_to_string(&mut contents).expect("Unable to read the file");

    c.bench_function("Parse input nom many0", |b| b.iter(|| {
        let _output = many0(day2::line_parser)(&contents);
    }
    ));
    c.bench_function("Parse input nom loop", |b| b.iter(|| {
        for range in &input_ranges {
            let _ = all_consuming(day2::line_parser)(range);
        }
    }
    ));
    c.bench_function("Parse input split", |b| b.iter(|| {
        for range in &input_ranges {
            let _ = day2::generate(range);
        }
    }
    ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);