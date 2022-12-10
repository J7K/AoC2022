use std::collections::HashSet;
use std::env;
use std::io::*;
use std::fs::*;

enum Direction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

fn read_cmd(line: &str) -> Direction
{
    let mut chars = line.chars();
    let dir = chars.next().unwrap();
    chars.next();
    let dist = chars.as_str().parse::<usize>().unwrap();
    match dir {
        'U' => Direction::Up(dist),
        'D' => Direction::Down(dist),
        'L' => Direction::Left(dist),
        'R' => Direction::Right(dist),
        _ => panic!("Invalid direction"),
    }
}

fn apply_cmds<'a>(start_pos:&(i32, i32), cmds: &mut impl Iterator<Item = &'a Direction>) -> HashSet<(i32, i32)>
{
    let mut visited = HashSet::new();
    let mut head_pos = *start_pos;
    let mut tail_pos = *start_pos;

    for cmd in cmds
    {
        let dist = match cmd {
            Direction::Up(dist) => *dist,
            Direction::Down(dist) => *dist,
            Direction::Left(dist) => *dist,
            Direction::Right(dist) => *dist,
        };

        for _ in 0..dist
        {
            head_pos = move_head(&head_pos, cmd);
            tail_pos = move_tail(&head_pos, &tail_pos);
            visited.insert(tail_pos);
        }
    }

    visited
}

fn apply_cmds_multi<'a>(start_pos:&(i32, i32), cmds: &mut impl Iterator<Item = &'a Direction>) -> HashSet<(i32, i32)>
{
    const ROPE_LENGTH: usize = 10;
    let mut visited = HashSet::new();
    let mut rope = vec![*start_pos; ROPE_LENGTH];

    for cmd in cmds
    {
        let dist = match cmd {
            Direction::Up(dist) => *dist,
            Direction::Down(dist) => *dist,
            Direction::Left(dist) => *dist,
            Direction::Right(dist) => *dist,
        };

        for _ in 0..dist
        {
            rope[0] = move_head(&rope[0], cmd);
            for i in 1..ROPE_LENGTH
            {
                rope[i] = move_tail(&rope[i-1], &rope[i]);
            }            
            visited.insert(*rope.last().unwrap());
        }
    }    

    visited
}

fn move_head(pos: &(i32, i32), dir: &Direction) -> (i32, i32)
{
    match dir {
        Direction::Up(_) => (pos.0, pos.1 +1),
        Direction::Down(_) => (pos.0, pos.1 -1),
        Direction::Left(_) => (pos.0 -1, pos.1),
        Direction::Right(_) => (pos.0 +1, pos.1)
    }
}

fn move_tail(head:&(i32, i32), tail:&(i32, i32)) -> (i32, i32)
{
    let delta = (head.0 - tail.0, head.1 - tail.1);    
    
    let move_vector= match delta {
        (0, 0) | (0, 1) | (0, -1) | (1, 0) | (1, 1) | (1, -1) | (-1, 0) | (-1, 1) | (-1, -1) => (0,0),
        (0, y) => (0, y/y.abs()),
        (x,0) => (x/x.abs(), 0),
        (x,y) => (x/x.abs(), y/y.abs()),        
    };

    (tail.0 + move_vector.0, tail.1 + move_vector.1)
}


fn main() {
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).unwrap(); 
    let mut data: String = String::new();
    BufReader::new(file).read_to_string(&mut data).expect("Failed to read file");
    let cmds: Vec<_> = data.lines().map(|line| read_cmd(line)).collect();
    
    let silver = apply_cmds(&(0,0), &mut cmds.iter()).len();
    println!("Silver: {}", silver);

    let gold = apply_cmds_multi(&(0,0), &mut cmds.iter()).len();
    println!("Gold: {}", gold);
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_suite_1() {
        let data = 
"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let cmds: Vec<_> = data.lines().map(|line| read_cmd(line)).collect();
        let visited = apply_cmds(&(0,0), &mut cmds.iter());
        assert_eq!(visited.len(), 13);
    }

    #[test]
    fn test_suite_2() {
        let data = 
"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        let cmds: Vec<_> = data.lines().map(|line| read_cmd(line)).collect();
        let visited = apply_cmds_multi(&(0,0), &mut cmds.iter());
        assert_eq!(visited.len(), 36);
    }
    
}