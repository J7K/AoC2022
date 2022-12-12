use std::collections::VecDeque;
use std::env;
use std::io::*;
use std::fs::*;
use regex::Regex;

struct Monkey
{
    inspect_count: usize,
    item_list: VecDeque<usize>,
    stress_mult: usize,
    test_var: usize,
    recipient_true: usize,
    recipient_false: usize,
    worry_fn: fn(usize, usize) -> usize,
}

impl Monkey
{
    fn new(data: &str) -> Self
    {
        let mut monkey = Monkey {
            inspect_count: 0,
            item_list: VecDeque::new(),
            stress_mult: 0,
            test_var: 0,
            recipient_true: 0,
            recipient_false: 0,
            worry_fn: |a, b| a + b,
        };

        let re_num = Regex::new(r"(\d+)").unwrap();
        let mut nums: Vec<_> = re_num.captures_iter(data).map(|c|c[1].parse::<usize>().unwrap()).collect();
        monkey.recipient_false = nums.pop().unwrap();
        monkey.recipient_true = nums.pop().unwrap();
        monkey.test_var = nums.pop().unwrap();
        monkey.stress_mult = nums.pop().unwrap();
        monkey.item_list.extend(nums.iter().skip(1));

        monkey
    }

    fn play_round(&mut self, modulo: usize) -> Vec<(usize, usize)>
    {
        let mut results = Vec::new();    
        while self.item_list.len() > 0
        {            
            self.inspect_count += 1;
            let item = self.item_list.pop_front().unwrap();            
            let mut worry_lvl = (self.worry_fn)(item, self.stress_mult);            
            worry_lvl %= modulo;
            if worry_lvl % self.test_var == 0
            {
                results.push((self.recipient_true, worry_lvl));
            }
            else
            {
                results.push((self.recipient_false, worry_lvl));
            }
        }


        results
    }
}


fn main() {
    const ROUNDS_TO_PLAY: usize = 10_000;
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).unwrap(); 
    let mut data: String = String::new();
    BufReader::new(file).read_to_string(&mut data).expect("Failed to read file");
    let mut monkeys: Vec<_> = data.split("\r\n\r\n").map(|m|Monkey::new(m)).collect();
    monkeys[0].worry_fn = |a, b| a * b;
    monkeys[1].worry_fn = |a, b| a * b;
    monkeys[4].worry_fn = |a, _| a * a;
    monkeys[4].item_list.push_back(76);
        
    let modulo = monkeys.iter().map(|m|m.test_var).product();
    for _ in 0..ROUNDS_TO_PLAY
    {
        for i in 0..monkeys.len()
        {
            let m = &mut monkeys[i];
            let results = m.play_round(modulo);
            for (dest, item) in results
            {
                monkeys[dest].item_list.push_back(item);
            }
        }
}

let mut counters:Vec<_> = monkeys.iter().map(|m|m.inspect_count).collect();
counters.sort();
counters.reverse();
let silver = counters[0] * counters[1];
println!("Silver: {}", silver);
}

