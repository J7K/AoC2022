use std::collections::VecDeque;
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

fn strip(c: char) -> char
{
    let b = match c
    {
        'S' => 'a',
        'E' => 'z',
            _ => c,
    };

    b
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
        if dest.is_some() 
        {
            let org = strip(self.get(pos));
            let dst = strip(self.get(dest.unwrap()));
            let diff = dst as i8 - org as i8;
            if diff > 1 
            { 
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

    fn find_start(&self) -> Option<(usize, usize)>
    {
        for y in 0..self.height
        {
            for x in 0..self.width
            {
                if self.get((x, y)) == 'S' { return Some((x, y)); }
            }
        }

        None
    }

    fn shortest_path(&self, x0: usize, y0: usize) -> Option<usize>
    {
        let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
        let mut visited = vec!(vec![false; self.height]; self.width);

        queue.push_back((x0, y0, 0));
        visited[x0][y0] = true;

        while(queue.len() > 0)
        {
            let (x, y, dist) = queue.pop_front().unwrap();
            if self.get((x, y)) == 'E' { return Some(dist); }
            let mut next = self.reach_from((x, y));
            next.retain(|(x, y)| !visited[*x][*y]);
            next.iter().for_each(|(x, y)| visited[*x][*y] = true);
            next.iter().for_each(|(x, y)| queue.push_back((*x, *y, dist + 1)));
        }
        
        None
    }

    fn shortest_all_paths(&self) -> Option<usize>
    {
        let mut starts = Vec::new();
        for y in 0..self.height
        {
            for x in 0..self.width
            {
                let current = self.get((x, y));
                if (current == 'S' || current == 'a') { starts.push((x, y)); }
            }
        }

        return starts.iter().map(|(x, y)| self.shortest_path(*x, *y)).filter(|x| x.is_some()).map(|x| x.unwrap()).min();
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).unwrap(); 
    let mut data: String = String::new();
    BufReader::new(file).read_to_string(&mut data).expect("Failed to read file");

    let map = Map::read_from_str(&data);
    let (x0, y0) = map.find_start().unwrap();
    let silver = map.shortest_path(x0, y0);
    println!("Silver: {}", silver.unwrap());

    let gold = map.shortest_all_paths();
    println!("Gold: {}", gold.unwrap());
}

