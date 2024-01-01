use std::{fs::File, io::BufReader};

use criterion::{criterion_group, criterion_main, Criterion};
use nom::combinator::all_consuming;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input_file = File::open("input.txt").unwrap();
    let input_ranges = day2::parse(BufReader::new(input_file)).unwrap();

    c.bench_function("Parse input", |b| b.iter(|| {
        for range in &input_ranges {
            let _ = all_consuming(day2::line_parser)(&range);
        }
    }
    ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);