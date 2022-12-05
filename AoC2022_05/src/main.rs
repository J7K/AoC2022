use std::env;
use std::io::*;
use std::fs::*;
use regex::Regex;

struct CraneOp
{
    nb_crates: usize,
    origin: usize,
    destination: usize
}

impl CraneOp
{
    fn new(nb_crates: usize, origin: usize, destination: usize) -> CraneOp
    {
        CraneOp 
        {
            nb_crates: (nb_crates),
            origin: (origin - 1) , 
            destination: (destination - 1)
        }
    }
}

struct CrateStacks
{
    stack_array: Vec<Vec<char>>,
}

impl CrateStacks 
{
    fn move_crates(&mut self, op: &CraneOp)
    {
        assert!(op.origin < self.stack_array.len());
        assert!(op.destination < self.stack_array.len());
        assert!(op.nb_crates <= self.stack_array[op.origin].len());

        for _ in 0..op.nb_crates
        {
            let tmp = self.stack_array[op.origin].pop().unwrap();
            self.stack_array[op.destination].push(tmp);
        }
    }

    fn move_crates_with_9001(&mut self, op: &CraneOp)
    {
        assert!(op.origin < self.stack_array.len());
        assert!(op.destination < self.stack_array.len());
        assert!(op.nb_crates <= self.stack_array[op.origin].len());

        let mut tmp: Vec<char> = Vec::new();
        for _ in 0..op.nb_crates
        {
            tmp.push(self.stack_array[op.origin].pop().unwrap());
        }
        for _ in 0..op.nb_crates
        {
            self.stack_array[op.destination].push(tmp.pop().unwrap());
        }
    }

    fn new(layout: &str) -> Self
    {   
        let indexed_crates = layout
            .split('\n')
            .into_iter()
            .rev()
            .skip(1)
            .flat_map(|line| {
                line.chars()
                    .skip(1)
                    .step_by(4)
                    .enumerate()
                    .filter(|(_, c)| *c != ' ')
            })
            .collect::<Vec<_>>();
        
        let mut stacks = vec![Vec::<_>::new(); indexed_crates.iter().map(|(i, _)| i).max().unwrap() + 1];

        for (i, c) in indexed_crates
        {
            stacks[i].push(c);
        }
       
        Self { stack_array: stacks }
    }

    fn top_crates(&self) -> String
    {
        self.stack_array.iter().map(|stack|stack.last().unwrap_or(&' ')).collect()
    }
    
}

fn main()
{
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).unwrap(); 
    let mut data: String = String::new();
    BufReader::new(file).read_to_string(&mut data).expect("Failed to read file");

    let input_split: Vec<_> = data.split("\r\n\r\n")
        .into_iter()
        .collect();

    assert!(input_split.len() == 2, "Invalid input data");
    let mut silver_stacks = CrateStacks::new(input_split[0]);

    let re = Regex::new(r".+\s(\d+).+\s(\d+).+\s(\d+)").unwrap();
    let ops: Vec<CraneOp> = re.captures_iter(input_split[1])
        .map(|cap|CraneOp::new(cap[1].parse::<usize>().unwrap(), cap[2].parse::<usize>().unwrap(), cap[3].parse::<usize>().unwrap()))
        .collect();

    for op in &ops
    {
        silver_stacks.move_crates(&op);
    }
    println!("Silver: {}", silver_stacks.top_crates());

    let mut gold_stacks = CrateStacks::new(input_split[0]);
    for op in &ops
    {
        gold_stacks.move_crates_with_9001(&op);
    }
    println!("Gold: {}", gold_stacks.top_crates());
}
