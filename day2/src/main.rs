use std::env;
use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let file = File::open(file_path)?;
    
    let reader = BufReader::new(file);
    const RADIX: u32 = 10;
    let mut sum = 0;

    for line in reader.lines() {
        let (mut b, mut r, mut g) : (u32, u32, u32) = (0, 0, 0);
        let sb: &str = &line?;
        let mut v: Vec<&str> = sb.rsplit(&[':', ',', ';'][..]).collect();
        v.truncate(v.len() - 1);

        for d in v {
            match d.replace(" ", "").as_bytes() {
                [f, l, color, ..] if (*l as char).to_digit(RADIX).is_some() => match color {
                    b'b' => b = max(b, (*f as char).to_digit(RADIX).unwrap() * 10 +
                     (*l as char).to_digit(RADIX).unwrap()),
                    b'r' => r = max(r, (*f as char).to_digit(RADIX).unwrap() * 10 +
                     (*l as char).to_digit(RADIX).unwrap()),
                    b'g' => g = max(g, (*f as char).to_digit(RADIX).unwrap() * 10 +
                     (*l as char).to_digit(RADIX).unwrap()),
                    _ => ()
                },
                [n, color, ..] => match color {
                    b'b' => b = max(b, (*n as char).to_digit(RADIX).unwrap()),
                    b'r' => r = max(r, (*n as char).to_digit(RADIX).unwrap()),
                    b'g' => g = max(g, (*n as char).to_digit(RADIX).unwrap()),
                    _ => ()
                },
                _ => ()
            }
        }
        sum += b * r * g;
    }

    println!("Sum: {}", sum);
    Ok(())
}
