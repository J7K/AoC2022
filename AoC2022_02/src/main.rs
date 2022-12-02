use std::collections::HashMap;
use std::env;
use std::io::*;
use std::fs::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).unwrap();
    let lines: Vec<_> = BufReader::new(file)
        .lines()
        .filter_map(|x|x.ok())
        .collect();

    let score_silver: u32 = lines.iter()
        .map(round_score_silver)
        .sum();

    let score_gold: u32 = lines.iter()
        .map(round_score_gold)
        .sum();

    
    println!("Silver Star - Final Score: {}", &score_silver);
    println!("Gold Star - Final Score: {}", &score_gold);
}

fn round_score_silver(round: &String) -> u32
{
    return *
    [ 
        ("A X", 4), ("A Y", 8), ("A Z", 3),
        ("B X", 1), ("B Y", 5), ("B Z", 9),
        ("C X", 7), ("C Y", 2), ("C Z", 6)
    ].into_iter()
    .collect::<HashMap<_,u32>>()
    .get(round.as_str())
    .unwrap();
}

fn round_score_gold(round: &String) -> u32
{
    return *
    [ 
        ("A X", 3), ("A Y", 4), ("A Z", 8),
        ("B X", 1), ("B Y", 5), ("B Z", 9),
        ("C X", 2), ("C Y", 6), ("C Z", 7)
    ].into_iter()
    .collect::<HashMap<_,u32>>()
    .get(round.as_str())
    .unwrap();
}