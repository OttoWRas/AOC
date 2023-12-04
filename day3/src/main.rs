use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let mut part : u32 = 1;
    if let Some(n) = &args[2].as_str().chars().nth(0) {
        if let Some(m) = n.to_digit(RADIX){
            part = m;
        }
    } 
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    const RADIX: u32 = 10;
    
    let mut symbols : HashMap<(usize, usize), char> = HashMap::new();
    let mut numbers : HashMap<(usize, usize), u32> = HashMap::new();
    let mut sum: u32 = 0;
    for (i, line) in reader.lines().enumerate() {
        let mut num_buff : Vec<u32> = Vec::new();
        let mut pos_buff : Option<(usize, usize)> = None;
        for (j, c) in line?.chars().enumerate() {
            match c {
                _ if c.to_digit(RADIX).is_some() => { if pos_buff.is_none() {
                    pos_buff = Some((i,j));
                }
                    num_buff.push(c.to_digit(RADIX).unwrap());
                },
                _ if c != '.' => { symbols.insert((i,j), c); 
                                if num_buff.len() > 0 {
                                    numbers.insert(pos_buff.unwrap(),
                                    num_buff.iter().fold(0, |acc, elem| acc * 10 + elem));
                                    pos_buff = None;
                                    num_buff.clear(); 
                                }
                            },
                _ => if num_buff.len() > 0 {
                    numbers.insert(pos_buff.unwrap(),
                    num_buff.iter().fold(0, |acc, elem| acc * 10 + elem));
                    pos_buff = None;
                    num_buff.clear(); 
                }
            }
        }
        if num_buff.len() > 0 {
            numbers.insert(pos_buff.unwrap(),
            num_buff.iter().fold(0, |acc, elem| acc * 10 + elem));
        }
    }
    if part == 1 {
        for x in &numbers {
            match x {
                ((m, n), u) => {
                    let k = if *u > 99 {4} else if *u > 9 {3} else {2};
                    let m_range = if *m > 0 { *m - 1 .. *m + 2 } else { 0..2 };
                    let n_range = if *n > 0 { *n - 1 .. *n + k } else { 0..k };
                    if m_range
                    .flat_map(move |m| n_range.clone()
                    .map(move |n| (m,n)))
                    .fold(false, |acc, (m,n)| acc || symbols.get(&(m,n)).is_some()) {
                        sum += *u;
                    }
                }
            }
        }
    } else {
        let mut gears : HashMap<(usize, usize), u32> = HashMap::new();
        for x in &numbers {
            match x {
                ((m, n), u) => {
                    let k = if *u > 99 {4} else if *u > 9 {3} else {2};
                    let m_range = if *m > 0 { *m - 1 .. *m + 2 } else { 0..2 };
                    let n_range = if *n > 0 { *n - 1 .. *n + k } else { 0..k };
                    let _ = m_range
                    .flat_map(move |m| n_range.clone()
                    .map(move |n| (m,n)))
                    .map(|(m,n)| match symbols.get(&(m,n)) {
                       Some(&'*') => { gears.insert((m,n), *u); } 
                       Some(_)    => {sum += *u; },
                       None => () 
                    });
                }
            }
        }
        //println!("{:?}",symbols);
        sum += &gears.iter().map(move |((_,_),s)| s ).sum();
    }
    println!("Sum: {}", sum);
    Ok(())
}