use std::env;
use std::cmp::*;
use std::io::*;
use std::fs::*;

use itertools::*;
use serde::*;

#[derive(Deserialize, Eq, PartialEq, Clone)]
#[serde(untagged)]
enum Packet
{
    Num(u32),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {       
        match (self, other) {
            (Self::List(a), Self::List(b)) => a.cmp(b),
            (Self::List(a), Self::Num(b)) => a.cmp(&vec![Self::Num(*b)]),
            (Self::Num(a), Self::List(b)) => vec![Self::Num(*a)].cmp(&b),
            (Self::Num(a), Self::Num(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}



fn main() {
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).unwrap(); 
    let mut data: String = String::new();
    BufReader::new(file).read_to_string(&mut data).expect("Failed to read file");

    let packets : Vec<Packet> = data.split("\r\n")
                                .filter(|x| x.len() > 0)
                                .map(|x| serde_json::from_str::<Packet>(x).unwrap())
                                .collect();

    let silver: usize = packets.iter()
                        .tuples()
                        .enumerate()
                        .filter(|(_, (x, y))| x < y)
                        .map(|(idx,_)| idx + 1)//Indices start at 1 in the problem description
                        .sum(); 

    println!("Silver: {}", silver);

    let dividers = vec![serde_json::from_str::<Packet>("[[2]]").unwrap(), serde_json::from_str::<Packet>("[[6]]").unwrap()];

    let mut packets_sorted = packets.clone();
    packets_sorted.extend(dividers.clone().into_iter());
    packets_sorted.sort();
    
    let gold: usize = packets_sorted.iter()
                .positions(|x| x == &dividers[0] || x == &dividers[1])
                .map(|idx| idx + 1)
                .product();
    println!("Gold: {}", gold);
    

}

