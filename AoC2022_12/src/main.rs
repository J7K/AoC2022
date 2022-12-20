use std::env;
use std::io::*;
use std::fs::*;

enum Direction
{
    North,
    South,
    East,
    West,
}

struct Map
{
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Map 
{
    fn read_from_str(data: &str) -> Self
    {
        let map: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect::<Vec<char>>()).collect();
        let width =map[0].len();
        let height = map.len();
        Map { map, width, height }
    }

    fn reach_from(&self, pos: (usize, usize)) -> Vec<(usize, usize)>
    {
        let mut result = Vec::new();
        if self.move_from(pos, Direction::North).is_some() { result.push(self.move_from(pos, Direction::North).unwrap()); }
        if self.move_from(pos, Direction::South).is_some() { result.push(self.move_from(pos, Direction::South).unwrap()); }
        if self.move_from(pos, Direction::East).is_some() { result.push(self.move_from(pos, Direction::East).unwrap()); }
        if self.move_from(pos, Direction::West).is_some() { result.push(self.move_from(pos, Direction::West).unwrap()); }

        result
    }

    fn move_from(&self, pos: (usize, usize), dir: Direction) -> Option<(usize, usize)>
    {
        let (x, y) = pos;
        let mut dest = match dir
        {
            Direction::North => if y == 0 { None } else { Some((x, y - 1)) },
            Direction::South => if (y + 1) == self.height { None } else { Some((x, y + 1)) },
            Direction::East => if (x + 1) == self.width { None } else { Some((x + 1, y)) },
            Direction::West => if x == 0 { None } else { Some((x - 1, y)) },
        };
        if dest.is_some() && (self.get(pos) != 'S' || self.get(dest.unwrap()) != 'E') 
        {
            if (self.get(dest.unwrap()) as u8 - self.get(pos) as u8) > 1 { 
                dest = None; 
            }
        }
        dest
    }

    fn get(&self, pos: (usize, usize)) -> char
    {
        let (x, y) = pos;
        self.map[y][x]
    }

    fn shortest_path(&self) -> Vec<(usize, usize)>
    {
        let mut path = Vec::new();
        let mut visited = Vec::new();
        let start = (0, 0);
        visited.push(start);
        self.find_path(start, &mut path, &mut visited);
        path
    }

    fn find_path(&self, pos: (usize, usize), path: &mut Vec<(usize, usize)>, visited: &mut Vec<(usize, usize)>) -> bool
    {
        let mut result = false;
        if self.get(pos) == 'E' { result = true; }
        else
        {
            let next = self.reach_from(pos);
            for n in next
            {
                if !visited.contains(&n)
                {
                    visited.push(n);
                    result = self.find_path(n, path, visited);
                    if result { break; }
                    visited.pop();
                }
            }
        }
        if result { path.push(pos); }
        result
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).unwrap(); 
    let mut data: String = String::new();
    BufReader::new(file).read_to_string(&mut data).expect("Failed to read file");

    let map = Map::read_from_str(&data);
    let path = map.shortest_path();
    dbg!(path);
}

