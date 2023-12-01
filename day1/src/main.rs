use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let file = File::open(file_path)?;
    
    let reader = BufReader::new(file);
    const RADIX: u32 = 10;
    let mut string_buffer: String = String::from(""); 
    
    let mut sum = 0;
    for line in reader.lines() {
        let mut f : Option<u32> = None;
        let mut l : Option<u32> = None;
        for c in line?.chars(){
            
            if let Some(n) = c.to_digit(RADIX) {
                if f == None {f = Some(n)} else {l = Some(n)}
                string_buffer.clear();
            } else {
                string_buffer.push(c);
            }
            
            if let Some(n) = match string_buffer.as_str() {
                s if s.contains("one") =>   Some(1), 
                s if s.contains("two") =>   Some(2),
                s if s.contains("three") => Some(3),
                s if s.contains("four") =>  Some(4),
                s if s.contains("five") =>  Some(5),
                s if s.contains("six") =>   Some(6),
                s if s.contains("seven") => Some(7),
                s if s.contains("eight") => Some(8),
                s if s.contains("nine") =>  Some(9),
                _ => None,
            } {
                if f == None {f = Some(n)} else {l = Some(n)}
                string_buffer.clear();
                string_buffer.push(c);
            }
        }
        sum += match (f, l) {
            (Some(n), Some(m)) => n * 10 + m,
            (Some(n),_) => n * 10 + n,
            (_, _) => 0,
        };
    }
    println!("Sum: {}", sum);
    Ok(())
}