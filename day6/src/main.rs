use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut part = 0;


    const RADIX: u32 = 10;
    let file_path = &args[1];
    if let Some(n) = &args[2].as_str().chars().nth(0) {
        if let Some(m) = n.to_digit(RADIX){
            part = m;
        }
    } 

    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut time = String::new();
    let mut distance = String::new();
    reader.read_line(&mut time)?;
    reader.read_line(&mut distance)?;
    let mut sum = 1;

    if part == 2 {

        let time = &(time.chars().filter(|c| !c.is_whitespace())
        .collect::<String>().split(':')).collect::<Vec<&str>>()
        .iter().filter_map(|x| x.parse::<u64>().ok()).collect::<Vec<u64>>();

        let distance = &(distance.chars().filter(|c| !c.is_whitespace())
        .collect::<String>().split(':')).collect::<Vec<&str>>()
        .iter().filter_map(|x| x.parse::<u64>().ok()).collect::<Vec<u64>>();

        sum = (0..time[0]).fold(0, |acc, x| if x*(time[0]-x) > distance[0] {acc + 1} else { acc });

    } else {
        let time = &(time.split_whitespace()
        .filter_map(|x| x.parse::<u64>().ok()).collect::<Vec<u64>>());
        let distance = &(distance.split_whitespace()
        .filter_map(|x| x.parse::<u64>().ok()).collect::<Vec<u64>>());
    
        for (i, t) in time.iter().enumerate() {
            sum *= (0..*t).fold(0, |acc, x| if x*(*t-x) > distance[i] {acc + 1} else { acc });
        }
    }

    println!("sum: {:?}",sum);
    Ok(())
}
