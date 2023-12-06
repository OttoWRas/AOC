use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let file = File::open(file_path)?;
    
    let reader = BufReader::new(file);
    //const RADIX: u32 = 10;
    let mut sum = 0;

    for line in reader.lines() {
        let line = line?;
        let nmd = line.split([':', '|']).collect::<Vec<&str>>();
        let win = nmd[1].split_whitespace().collect::<Vec<&str>>();
        sum += nmd[2].split_whitespace().collect::<Vec<&str>>()
        .iter().fold(0, |acc, x| if win.contains(x) {
            if acc == 0 { 1 } else { acc * 2 } } else {acc} );
    }

    println!("Sum: {}", sum);
    Ok(())
}
