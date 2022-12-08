use std::collections::HashSet;
use std::env;
use std::io::*;
use std::fs::*;

enum Direction { North, South, East,West, }

struct Forrest
{
    trees: Vec<Vec<u8>>,
    max_x: usize,
    max_y: usize,
}

impl Forrest
{
    fn new(data: &str) -> Self
    {
        let mut forrest = Forrest 
        {
            trees: vec![],
            max_x: 0,
            max_y: 0,
         };
        forrest.trees.extend(data.lines().map(|line| line.chars().map(|c| c as u8 - '0' as u8).collect()));
        forrest.max_x = forrest.trees[0].len();
        forrest.max_y = forrest.trees.len();

        forrest
    }

    fn move_pos(&self, pos: (usize, usize), dir: &Direction) -> Option<(usize, usize)>
    {
        let (x, y) = pos;
        match dir
        {
            Direction::North => { if y == 0 { None } else { Some( (x, y - 1) )} },
            Direction::South => { if (y + 1) == self.max_y { None } else { Some( (x, y + 1) )} },
            Direction::East => { if (x + 1) == self.max_x { None } else { Some( (x + 1, y) )} },
            Direction::West => { if x == 0  { None } else { Some( (x - 1, y) )} },
        }
    }

    fn height_at(&self, pos: &(usize, usize)) -> u8
    {
        self.trees[pos.1][pos.0]
    }

    fn visible_from(&self, init_pos: &(usize, usize), dir: &Direction) -> Vec<(usize, usize)>
    {
        let mut visible = Vec::new();
        let mut pos = init_pos.clone();
        let mut max_height = self.height_at(&pos);
        visible.push(pos);

        while let Some(new_pos) = self.move_pos(pos, dir)
        {
            pos = new_pos;
            let height = self.height_at(&pos);
            if height > max_height
            {
                max_height = height;
                visible.push(pos);
            }
        }
        visible
    }

    fn viewing_dist(&self, pos: &(usize, usize), dir: &Direction) -> usize
    {
        let mut dist = 0;
        let mut current = pos.clone();
        while let Some(new_pos) = self.move_pos(current, dir)
        {
            current = new_pos;
            dist += 1;
            if self.height_at(&current) >= self.height_at(pos)
            {
                break;
            }
        }
        dist
    }

    fn scenic_score(&self, pos: &(usize, usize)) -> usize
    {
        self.viewing_dist(pos, &Direction::North) 
        * self.viewing_dist(pos, &Direction::South) 
        * self.viewing_dist(pos, &Direction::East) 
        * self.viewing_dist(pos, &Direction::West)
    }

    fn scenic_scores(&self) -> Vec<Vec<usize>>
    {
        let mut scores = vec![vec![0; self.max_x]; self.max_y];
        for y in 0..self.max_y
        {
            for x in 0..self.max_x
            {
                scores[y][x] = self.scenic_score(&(x, y));
            }
        }
        scores
    }

    fn visible_from_all(&self) -> HashSet<(usize, usize)>
    {
        let mut visible_set: HashSet<(usize, usize)> = HashSet::new();
        for x in 0..self.max_x
        {
            visible_set.extend(self.visible_from(&(x, 0), &Direction::South));   
            visible_set.extend(self.visible_from(&(x, self.max_y - 1), &Direction::North));
        }
        for y in 0..self.max_y
        {
            visible_set.extend(self.visible_from(&(self.max_x - 1, y), &Direction::West));
            visible_set.extend(self.visible_from(&(0, y), &Direction::East));
        }

        visible_set
    }


}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).unwrap(); 
    let mut data: String = String::new();
    BufReader::new(file).read_to_string(&mut data).expect("Failed to read file");
    
    let forrest = Forrest::new(data.as_str());
    let silver = forrest.visible_from_all().iter().count();
    println!("Silver: {}", silver);

    let scores = forrest.scenic_scores();
    let gold = scores.iter().flat_map(|r|r.iter()).max().unwrap();
    println!("Gold: {}", gold);
}

#[cfg(test)]
mod test
{
    use super::*;
    #[test]
    fn test_suite_1()
    {
        let data = 
"30373
25512
65332
33549
35390";
        let forrest = Forrest::new(data);
        let nb_visible = forrest.visible_from_all().iter().count();
        assert_eq!(nb_visible, 21);
    }
    #[test]
    fn test_suite_2()
    {
        let data =
"30373
25512
65332
33549
35390";
        let forrest = Forrest::new(data);      
        assert_eq!(forrest.scenic_score(&(2,1)), 4);
        assert_eq!(*forrest.scenic_scores().iter().flat_map(|r|r.iter()).max().unwrap(), 8);
    }
}