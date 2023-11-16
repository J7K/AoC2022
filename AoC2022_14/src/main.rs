use std::env;
use std::collections::*;
use std::io::*;
use std::fs::*;

struct Cave{    
    rocks: HashMap<usize, HashSet<usize>>,
    floor: usize,
}

impl Cave
{
    fn new() -> Self {
        Cave{
            rocks: HashMap::new(),
            floor: usize::MAX,
        }
    }

    fn add_rock(&mut self, x: usize, y: usize) {
        let set = self.rocks.get_mut(&x);
        if set.is_none() {
            self.rocks.insert(x, HashSet::new());
        }
        self.rocks.get_mut(&x).unwrap().insert(y);        
    }

    fn add_rock_path(&mut self, path: &Vec<(usize, usize)>) {
        assert!(path.len() > 1);
        let mut origin = path[0];

        for i in 1..path.len() {
            let next = path[i];          
            if next.0 == origin.0 {
                let (start, stop) = if origin.1 < next.1 { (origin.1, next.1) } else { (next.1, origin.1) };
                for y in start..=stop {
                    self.add_rock(origin.0, y);
                }
            }
            else if next.1 == origin.1 {
                let (start, stop) = if origin.0 < next.0 { (origin.0, next.0) } else { (next.0, origin.0) };
                for x in start..=stop {
                    self.add_rock(x, origin.1);
                }
            }
            else {
                panic!("Invalid path");
            }
            origin = next;
        }

        self.floor = self.floor_ord();
    }
    
    fn drop_rock(&mut self, x: usize, y: usize) -> Option<(usize, usize)> {
        let set = self.rocks.get_mut(&x);
        if set.is_some() {
            let contact_y = set.unwrap().iter().filter(|current| y < **current).min();
            if contact_y.is_some()
            {
                return Some( (x, *contact_y.unwrap()) );
            }
        }

        None
    }

    fn drop_rock_with_floor(&mut self, x: usize, y: usize) -> (usize, usize) {
        let set = self.rocks.get_mut(&x);
        if set.is_some() {
            let contact_y = set.unwrap().iter().filter(|current| y < **current).min();
            if contact_y.is_some() {
                return (x, *contact_y.unwrap());
            }
        }
        return (x, self.floor);
    }

    fn has_rock_at(&self, x: usize, y: usize) -> bool {
        let set = self.rocks.get(&x);
        if set.is_some() {
            return set.unwrap().contains(&y);
        }

        false
    }

    fn has_rock_at_with_floor(&self, x: usize, y: usize) -> bool {
        if y == self.floor {
            return true;
        }
        else {
            return self.has_rock_at(x, y);
        }
    }

    fn floor_ord(&self) -> usize
    {
        self.rocks.iter().map(|(_, set)| set.iter().max().unwrap()).max().unwrap() +2
    }

    fn sim_sand(&mut self, drop_abs: usize) -> bool {
        let mut overflow = false;

        let mut drop_point = (drop_abs, 0);
        loop {            
            let contact = self.drop_rock(drop_point.0, drop_point.1);
            if contact.is_some() {
                drop_point = contact.unwrap();
                let bottom_left_neighbour =  (drop_point.0 - 1, drop_point.1);
                let bottom_right_neighbour = (drop_point.0 + 1, drop_point.1);

                if !self.has_rock_at(bottom_left_neighbour.0, bottom_left_neighbour.1) {
                    drop_point = bottom_left_neighbour;
                }
                else if !self.has_rock_at(bottom_right_neighbour.0, bottom_right_neighbour.1) {
                    drop_point = bottom_right_neighbour;
                }
                else {
                    self.add_rock(drop_point.0, drop_point.1-1);
                    break;
                }
            }
            else {
                overflow = true;
                break;
            }
        }

        overflow
    }

    fn sim_sand2(&mut self, drop_abs: usize) -> bool {
        let mut filled = false;

        let mut drop_point = (drop_abs, 0);
        loop {            
            let contact = self.drop_rock_with_floor(drop_point.0, drop_point.1);
            let bottom_left_neighbour =  (contact.0 - 1, contact.1);
            let bottom_right_neighbour = (contact.0 + 1, contact.1);

            if !self.has_rock_at_with_floor(bottom_left_neighbour.0, bottom_left_neighbour.1) {
                drop_point = bottom_left_neighbour;
            }
            else if !self.has_rock_at_with_floor(bottom_right_neighbour.0, bottom_right_neighbour.1) {
                drop_point = bottom_right_neighbour;
            }
            else {
                self.add_rock(contact.0, contact.1-1);
                filled = contact.1 == 1;
                break;
            }
        }

        filled
    }
}

fn string_to_path(s: &str) -> Vec<(usize, usize)>
{
    let mut string = String::from(s);
    string.retain(|c| !c.is_whitespace());
    
    string.as_str().split("->").map (|s| {
        let mut split = s.split(",");
        let x = split.next().unwrap().parse::<usize>().unwrap();
        let y = split.next().unwrap().parse::<usize>().unwrap();
        (x, y)
    }).collect::<Vec<(usize, usize)>>()
}


fn main() {
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).unwrap(); 
    let mut data: String = String::new();
    BufReader::new(file).read_to_string(&mut data).expect("Failed to read file");

    let rock_paths = data.split("\n").map(|s| string_to_path(s)).collect::<Vec<Vec<(usize, usize)>>>();

    let mut cave = Cave::new();
    rock_paths.iter().for_each(|p| cave.add_rock_path(p));
    
    let mut overflow = false;
    let mut silver = 0;
    while !overflow {
        overflow = cave.sim_sand(500);
        silver += 1;
    }
    silver -= 1;

    println!("Silver: {}", silver);

    let mut cave2 = Cave::new();
    rock_paths.iter().for_each(|p| cave2.add_rock_path(p));

    let mut filled = false;
    let mut gold = 0;
    while !filled {
        filled = cave2.sim_sand2(500);
        gold += 1;
    }

    println!("Gold: {}", gold);
}

