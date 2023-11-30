use std::env;
use std::io::*;
use std::fs::*;
use regex::Regex;

#[derive(Debug)]
struct ScannedArea
{
    sensor:(i32, i32),
    radius:i32,
}

impl ScannedArea
{
    fn new(sensor:(i32, i32), nearest_beacon:(i32, i32)) -> Self
    {
        Self { sensor: sensor, radius: l1_dist(sensor, nearest_beacon) }
    }

    fn from_str(line: &str) -> Self
    {
       
        let re: Regex = Regex::new(r"(-?\d+)").unwrap();
        let coords: Vec<_> = re.captures_iter(line).map(|c|c[1].parse::<i32>().unwrap()).collect();

        assert!(coords.len() == 4, "Invalid input line");
        Self::new((coords[0], coords[1]), (coords[2], coords[3]))
    }

    fn contains(&self, beacon:(i32, i32)) -> bool
    {
        l1_dist(self.sensor, beacon) <= self.radius
    }
}

fn l1_dist(a:(i32, i32), b:(i32, i32)) -> i32
{
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn isec (x: i32, y: i32, areas: &Vec<ScannedArea>) -> bool
{
    for area in areas
    {
        if area.contains((x, y))
        {
            return true;
        }
    }
    false    
}

fn main() {
   let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).unwrap(); 
    let mut data: String = String::new();
    BufReader::new(file).read_to_string(&mut data).expect("Failed to read file");
    
    let mut scanned_areas: Vec<_> = data.as_str().lines().map(ScannedArea::from_str).collect();

    let silver_row: i32 = 10;
    let min_x = scanned_areas.iter().map(|a|a.sensor.0).min().unwrap();
    let max_x = scanned_areas.iter().map(|a|a.sensor.0).max().unwrap();

    dbg!(min_x, max_x);



    //let silver = (min_x..=max_x).map(|x| !isec(x.try_into().unwrap(), silver_row, &scanned_areas)).filter(|&b|b).count();
//    println!("Silver: {}", silver);
}