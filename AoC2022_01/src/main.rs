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

    
    // Compute calorie payload per elf
    let mut accum: u32 = 0;
    let mut elves_payload: Vec<u32> = Vec::new();
    for line in lines  
    {
        if line.is_empty()
        {
            elves_payload.push(accum);
            accum = 0;
        }
        else
        {
            accum += line.parse::<u32>().unwrap();
        }
    }
    // Account for end of file
    elves_payload.push(accum);

    // Find max carrying elf
    let max_payload = elves_payload.iter().max().unwrap();

    println!("Max carrying elf: {}", &max_payload);


    assert!(elves_payload.len() >=3, "More than 3 elves are required as input" );
    elves_payload.sort();
    elves_payload.reverse();
    let total_payload_top3 = elves_payload[0] + elves_payload[1] + elves_payload[2];

    println!("Payload of top3 carrying elves: {}", total_payload_top3);

}

