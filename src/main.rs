extern crate utf8parse;
use utf8parse::Receiver;

use std::env;
use std::fs::File;
use std::io::Read;
use std::time::{Duration, Instant};

struct CounterReceiver {
    ctr: usize,
}

impl CounterReceiver {
    pub fn new() -> Self {
        CounterReceiver { ctr: 0 }
    }
}

impl Receiver for CounterReceiver {
    fn codepoint(&mut self, _: char) {
        self.ctr += 1;
    }
    fn invalid_sequence(&mut self) {
        panic!("This is a performance benchmark. Use only valid UTF-8");
    }
}

trait BenchableUtf8Parser {
    fn parse(bytes: &[u8]) -> usize;
}

struct BenchableTableParser;

impl BenchableUtf8Parser for BenchableTableParser {
    fn parse(bytes: &[u8]) -> usize {
        let mut p = utf8parse::Parser::new();
        let mut r = CounterReceiver::new();
        for b in bytes.iter() {
            p.advance(&mut r, *b);
        }
        r.ctr
    }
}

struct BenchableStdlibParser;

impl BenchableUtf8Parser for BenchableStdlibParser {
    fn parse(bytes: &[u8]) -> usize {
        let chars_it = unsafe {
            std::str::from_utf8_unchecked(bytes).chars()
        };
        chars_it.count()
    }
}

fn bench1<B:BenchableUtf8Parser>(data: &[u8]) -> (Duration,usize) {
    let start = Instant::now();
    let parsed_len = B::parse(data);
    (start.elapsed(),parsed_len)
}

fn bench<B:BenchableUtf8Parser>(name: &str, data: &[u8]) {
    let results : Vec<(Duration,usize)> = (0..5).map(|_| bench1::<B>(data)).collect();
    let mut times : Vec<Duration> = results.iter().map(|tup| tup.0).collect::<Vec<Duration>>();
    times.sort();
    let median_time = times[times.len()/2];
    let parsed_len = results[0].1;
    println!("Parser \"{}\" needed a median {}.{:0>9} seconds to parse {} characters.", name, median_time.as_secs(), median_time.subsec_nanos(), parsed_len);
}

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: {} textfile", args[0]);
    } else {
        let mut data : Vec<u8> = Vec::new();
        {
            let mut input_file = File::open(&args[1]).unwrap();
            let flen = input_file.read_to_end(&mut data).unwrap();
            println!("Read {} bytes.", flen);
        } // close input file
        bench::<BenchableTableParser>("tbl", &data);
        bench::<BenchableStdlibParser>("std", &data);
    }
}
