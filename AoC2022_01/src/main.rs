use std::env;
use std::io::*;
use std::fs::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).expect("Failed to open file"); 
    let mut data: String = String::new();
    BufReader::new(file).read_to_string(&mut data).expect("Failed to read file");

    let mut elves_payloads: Vec<usize> = data.split("\r\n\r\n").into_iter()
        .map(|s| s.split("\r\n").collect::<Vec<_>>())
        .map(|s| s.iter().map(|s| s.parse::<usize>()).filter_map(std::result::Result::ok).sum())
        .collect();        

    let max_payload = elves_payloads.iter().max().unwrap();
    println!("Silver: {}", &max_payload);

    assert!(elves_payloads.len() >=3, "More than 3 elves are required as input" );
    elves_payloads.sort();
    elves_payloads.reverse();
    let total_payload_top3 = elves_payloads[0] + elves_payloads[1] + elves_payloads[2];

    println!("Gold: {}", total_payload_top3);
}

