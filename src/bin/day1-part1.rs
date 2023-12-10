// Work on part 1

// use std::env;
use std::fs::*;
use std::io::*;

fn main() {
    let _ = part1();
}

fn part1() -> Result<()> {
    let f = File::open("inputs/day1.txt")?;
    let reader = BufReader::new(f);


    let mut number = 0;
    for line in reader.lines() {
        let mut first = ' ';
        let mut last = ' ';
        for c in line.unwrap().chars() {
            if c.is_numeric() {
                if first == ' ' {
                    first = c;
                }
                last = c;
            }
        }
        // next line
        let val: u32 = (first.to_string() + &last.to_string()).parse().unwrap();
        number += val;
    }

    println!("{number}");
    Ok(())
}
