use std::env;
use std::io::*;
use std::fs::*;

struct Forrest
{
    trees: Vec<Vec<u8>>,
}

impl Forrest
{
    fn new(data: &str) -> Self
    {
        let mut forrest = Forrest { trees: vec![] };
        forrest.trees.extend(data.lines().map(|line| line.chars().map(|c| c as u8 - '0' as u8).collect()));

        forrest
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).unwrap(); 
    let mut data: String = String::new();
    BufReader::new(file).read_to_string(&mut data).expect("Failed to read file");
    
    let forrest = Forrest::new(data.as_str());
    dbg!(forrest.trees);
}

