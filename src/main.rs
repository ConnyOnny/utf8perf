extern crate utf8parse;
use utf8parse::Receiver;

use std::env;
use std::fs::File;
use std::io::Read;
use std::time::{Duration, Instant};

struct BufferReceiver {
    data: Vec<char>,
}

impl BufferReceiver {
    pub fn new() -> Self {
        BufferReceiver { data: vec![] }
    }
    pub fn done(self) -> Vec<char> {
        self.data
    }
}

impl Receiver for BufferReceiver {
    fn codepoint(&mut self, c: char) {
        self.data.push(c);
    }
    fn invalid_sequence(&mut self) {
        panic!("This is a performance benchmark. Use only valid UTF-8");
    }
}

trait BenchableUtf8Parser {
    fn parse(bytes: &[u8]) -> Vec<char>;
}

struct BenchableTableParser;

impl BenchableUtf8Parser for BenchableTableParser {
    fn parse(bytes: &[u8]) -> Vec<char> {
        let mut p = utf8parse::Parser::new();
        let mut r = BufferReceiver::new();
        for b in bytes.iter() {
            p.advance(&mut r, *b);
        }
        r.done()
    }
}

struct BenchableStdlibParser;

impl BenchableUtf8Parser for BenchableStdlibParser {
    fn parse(bytes: &[u8]) -> Vec<char> {
        let mut ret = Vec::new();
        let chars_it = unsafe {
            std::str::from_utf8_unchecked(bytes).chars()
        };
        for c in chars_it {
            ret.push(c);
        }
        ret
    }
}

fn bench1<B:BenchableUtf8Parser>(data: &[u8]) -> (Duration,usize) {
    let start = Instant::now();
    let parsed = B::parse(data);
    (start.elapsed(),parsed.len())
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
        bench::<BenchableTableParser>("table", &data);
        bench::<BenchableStdlibParser>("stdlib", &data);
    }
}
