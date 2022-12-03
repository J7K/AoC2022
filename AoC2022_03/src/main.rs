use std::collections::HashSet;
use std::env;
use std::io::*;
use std::fs::*;


fn main() {
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).unwrap();
    let lines: Vec<_> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect();


    let silver_ans: usize = lines.clone().into_iter()
        .map(isolate_common_char)
        .map(convert_to_priority)
        .sum();
        
    println!("Silver answer: {}", silver_ans);

    let gold_ans: usize = lines
        .chunks(3)
        .map(isolate_badge)
        .map(convert_to_priority)
        .sum();

    println!("Gold answer: {}", gold_ans);
}

fn convert_to_priority(mut c: char) -> usize
{
    let mut accum: usize = 1;
    if c.is_uppercase() 
    {
        accum += 26;
        c.make_ascii_lowercase()
    }
    accum += c as usize - 'a' as usize;

    return accum;
}

fn isolate_common_char(line: String) -> char
{
    let (first_compartment, second_compartment) = line.split_at(line.len() / 2);
    let set = second_compartment.chars().collect::<HashSet<char>>();
    return first_compartment.chars()
                            .filter(|x|set.contains(x))
                            .collect::<String>()
                            .chars()
                            .next()
                            .unwrap();
}

fn isolate_badge(chunk: &[String]) -> char
{
    assert!(chunk.len() == 3);
    let set1 = chunk[0].chars().collect::<HashSet<char>>();
    let set2 = chunk[1].chars().collect::<HashSet<char>>();
    return chunk[2].chars()
           .filter(|x|set1.contains(x) && set2.contains(x))
           .collect::<String>()
           .chars()
           .next()
           .unwrap();
}