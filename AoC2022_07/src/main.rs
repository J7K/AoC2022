use std::env;
use std::io::*;
use std::fs::*;

enum FsCmd
{
    CdBack,
    CdIn,
    File(usize),
    Nop,
}

struct FsNode 
{ 
    size: usize,
    children: Vec<FsNode> 
}

impl FsNode {
    fn build<'a>(cmds: &mut impl Iterator<Item = &'a FsCmd>) -> FsNode {
        let mut node = FsNode { size: 0, children: vec![] };
        while let Some(cmd) = cmds.next() {
            match cmd {
                FsCmd::CdBack => break,
                FsCmd::CdIn => { 
                    node.children.push(Self::build(cmds));
                    node.size += node.children.last().unwrap().size;
                },
                FsCmd::File(size) => node.size += size,
                FsCmd::Nop => (),
            }
        }
        node
    }

    fn traverse(&self) -> Box<dyn Iterator<Item = &Self> + '_> {
        Box::new(self.children.iter().flat_map(Self::traverse).chain([self]))
    }
}

fn parse_line(line: &str) -> FsCmd {
    if line.starts_with("$ cd ..") {
        return FsCmd::CdBack;
    }
    else if line.starts_with("$ cd /") {
        return FsCmd::Nop;
    }
    else if line.starts_with("$ cd") {
        return FsCmd::CdIn;
    }
    else if line.starts_with("dir") {
        return FsCmd::Nop;
    }
    else if line.starts_with("$ ls") {
        return FsCmd::Nop;
    }
    else {
        let mut parts = line.split(' ');
        let size = parts.next().unwrap().parse::<usize>().unwrap();
        return FsCmd::File(size);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).unwrap(); 
    let mut data: String = String::new();
    BufReader::new(file).read_to_string(&mut data).expect("Failed to read file");
    
    let cmds: Vec<_> = data.as_str().lines().map(parse_line).collect();
    let root = FsNode::build(&mut cmds.iter());
    
    let sizes: Vec<_> = root.traverse().map(|d| d.size).collect();

    let silver = sizes.iter().filter(|&&size| size <= 100000).sum::<usize>();
    println!("Silver: {}", silver);

    assert!( (70000000 - root.size) < 30000000, "No need to erase folder");
    let min_folder_to_delete_size = 30000000 - (70000000 - root.size);
    
    let gold = sizes.iter().filter(|&&size| size >= min_folder_to_delete_size).min().unwrap();
    println!("Gold: {}", gold);
}
