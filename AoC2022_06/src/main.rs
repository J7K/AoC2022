use std::env;
use std::collections::VecDeque;
use std::io::*;
use std::fs::*;

fn find_start_packet_marker(datastream: &str, marker_size: usize) -> usize
{
    assert!(datastream.len() > marker_size);
    let data: Vec<_> = datastream.chars().collect();
    let mut buffer: VecDeque<char> = VecDeque::new();
    let mut index = 0;

    while index < datastream.len()
    {
        let current = data[index];
        if buffer.contains(&current)
        {
            while buffer.pop_front() != Some(current) { /* EMPTY */}
        }

        buffer.push_back(current);
  
        if buffer.len() == marker_size
        {
            break;
        }
        index += 1;
    }
    index += 1;
    return index;
}

fn main() 
{
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).unwrap(); 
    let mut data: String = String::new();
    BufReader::new(file).read_to_string(&mut data).expect("Failed to read file");

    let silver = find_start_packet_marker(&data, 4);
    println!("Silver: {}", silver);

    let gold = find_start_packet_marker(&data, 14);
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
        assert_eq!(find_start_packet_marker(DATA, 4), 5);
    }

    #[test]
    fn test_start_index_2()
    {
        const DATA: &str = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(find_start_packet_marker(DATA, 4), 6);
    }       
    
    #[test]
    fn test_start_index_3()
    {
        const DATA: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(find_start_packet_marker(DATA, 4), 10);
    }

    #[test]
    fn test_start_index_4()
    {
        const DATA: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(find_start_packet_marker(DATA, 4), 11);
    }

    #[test]
    fn test_start_index_5()
    {
        const DATA: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(find_start_packet_marker(DATA, 14), 19);
    }

    #[test]
    fn test_start_index_6()
    {
        const DATA: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(find_start_packet_marker(DATA, 14), 23);
    }
    
    #[test]
    fn test_start_index_7()
    {
        const DATA: &str = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(find_start_packet_marker(DATA, 14), 23);
    }

    #[test]
    fn test_start_index_8()
    {
        const DATA: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(find_start_packet_marker(DATA, 14), 29);
    }
 
    #[test]
    fn test_start_index_9()
    {
        const DATA: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(find_start_packet_marker(DATA, 14), 26);
    }
}