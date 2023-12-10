// Work on part 1

// use std::env;
use std::fs::*;
use std::io::*;

const NUMBERS_ARRAY: [(&str,i32); 9] = [
    ("one",   1),
    ("two",   2),
    ("three", 3),
    ("four",  4),
    ("five",  5),
    ("six",   6),
    ("seven", 7),
    ("eight", 8),
    ("nine",  9),
];


fn main() {
    let _ = part1();
}

fn part1() -> Result<()> {
    let f = File::open("inputs/day1.txt")?;
    let reader = BufReader::new(f);


    let mut sum = 0;
    for line in reader.lines() {
        let number = parse_line_number(&line.unwrap());
        sum += number;
    }

    println!("{sum}");
    Ok(())
}


fn parse_line_number(line: &str) -> i32 {
    let mut nums = Vec::with_capacity(10);
    let mut count: usize = 0;

    for (index, c) in line.char_indices() {
        if c.is_numeric() {
            // println!("number: {}", c);
            nums.push(c.to_string());
            count += 1;
        } else if c.is_alphabetic() {
            for (string, number) in NUMBERS_ARRAY {
                if line.len() >= index + string.len() {
                    let opt = line.get(index..index + string.len());
                    match opt {
                        Some(slice) => {
                            if slice == string {
                                // Match!
                                // println!("string: {}", number);
                                nums.push(number.to_string());
                            }
                        }
                        None => (),
                    }
                }
            }
        }
    }

    let n: i32 = (nums[0].clone() + &nums[nums.len() - 1]).parse().unwrap();
    println!("Line Number: {}", n);
    n
}