use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let file = File::open(file_path)?;
    
    let reader = BufReader::new(file);
    let mut seeds = Vec::new();
    let mut n_seeds = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i == 0 {
            seeds = line.split([':', ' '])
            .filter_map(|x| x.parse::<i64>().ok())
            .collect::<Vec<i64>>();
            n_seeds = seeds.clone();
            continue;
        }

        if line.contains(':') || line.trim().is_empty() {
            if i != 1 && line.trim().is_empty(){
                seeds = n_seeds.clone();
            }
            continue;
        }

        let nums = line.split([' '])
        .filter_map(|x| x.parse::<i64>().ok())
        .collect::<Vec<i64>>();
        let range = match nums[..] {
            [a, b, c, ..] => (b..b+c, a-b),
            _ => (0..0, 0)
        };
        
        for i in 0 .. seeds.len() {
            if range.0.contains(&seeds[i]) {
                n_seeds[i] = range.1 + seeds[i];
            }
        }
    }

    println!("Sum: {:?}", n_seeds.iter().min());
    Ok(())
}
