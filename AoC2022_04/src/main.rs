use std::env;
use std::io::*;
use std::fs::*;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).unwrap(); 
    let mut data: String = String::new();
    BufReader::new(file).read_to_string(&mut data).expect("Failed to read file");
    
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let bounds :Vec<_> = re.captures_iter(data.as_str())
        .map(|cap|cap.iter().skip(1).map(|m| m.unwrap().as_str().parse::<usize>().unwrap()).collect::<Vec<_>>())
        .flatten()
        .collect();

    let silver_ans = bounds.chunks(4)
        .filter(|chunk|contains(chunk[0], chunk[1], chunk[2], chunk[3]))
        .count();
    println!("Silver: {}", silver_ans);
    
    let gold_ans = bounds.chunks(4)
        .filter(|chunk|overlaps(chunk[0], chunk[1], chunk[2], chunk[3]))
        .count();
    println!("Gold: {}", gold_ans);
}

fn contains<T: std::cmp::PartialOrd>(x1: T, y1: T, x2: T, y2: T) -> bool
{
  return (x1 <= x2 && y1 >= y2) || (x2 <= x1 && y2 >= y1);
}

fn overlaps<T: std::cmp::PartialOrd>(x1: T, y1: T, x2: T, y2: T) -> bool
{
  return ((x1 <= y2) && (y1 >= x2)) || ((x2 <= y1) && (y2 >= x1));
}