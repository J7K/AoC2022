use std::env;
use std::collections::VecDeque;
use std::io::*;
use std::fs::*;

fn index_after_marker(datastream: &str, marker_size: usize) -> usize
{
    assert!(datastream.len() > marker_size);
    let data = datastream.as_bytes();
    let mut buffer: VecDeque<u8> = VecDeque::new();
    let mut index = 0;

    while (buffer.len() < marker_size) && (index < datastream.len())
    {
        let current = data[index];
        index += 1;

        if buffer.contains(&current)
        {
            while buffer.pop_front() != Some(current) { /* EMPTY */}
        }

        buffer.push_back(current);
    }
    return index;
}

fn main() 
{
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).unwrap(); 
    let mut data: String = String::new();
    BufReader::new(file).read_to_string(&mut data).expect("Failed to read file");

    let silver = index_after_marker(&data, 4);
    println!("Silver: {}", silver);

    let gold = index_after_marker(&data, 14);
    println!("Gold: {}", gold);
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_start_index_1()
    {
        const DATA: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(index_after_marker(DATA, 4), 5);
    }

    #[test]
    fn test_start_index_2()
    {
        const DATA: &str = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(index_after_marker(DATA, 4), 6);
    }       
    
    #[test]
    fn test_start_index_3()
    {
        const DATA: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(index_after_marker(DATA, 4), 10);
    }

    #[test]
    fn test_start_index_4()
    {
        const DATA: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(index_after_marker(DATA, 4), 11);
    }

    #[test]
    fn test_start_index_5()
    {
        const DATA: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(index_after_marker(DATA, 14), 19);
    }

    #[test]
    fn test_start_index_6()
    {
        const DATA: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(index_after_marker(DATA, 14), 23);
    }
    
    #[test]
    fn test_start_index_7()
    {
        const DATA: &str = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(index_after_marker(DATA, 14), 23);
    }

    #[test]
    fn test_start_index_8()
    {
        const DATA: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(index_after_marker(DATA, 14), 29);
    }
 
    #[test]
    fn test_start_index_9()
    {
        const DATA: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(index_after_marker(DATA, 14), 26);
    }
}