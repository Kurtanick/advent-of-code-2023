use std::{char, i32};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::cmp::{min,max};
use std::collections::HashMap;


fn read_file_to_2d_vector(file_path: &str) -> Result<Vec<Vec<char>>, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut result = Vec::new();

    for line in reader.lines() {
        let chars: Vec<char> = line?.chars().collect();
        result.push(chars);
    }

    Ok(result)
}

fn is_gear(ch: &char) -> bool {
    ch.to_owned() == '*'
}

fn parse_schematic(schematic: Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    let mut gear_map: HashMap<(usize,usize), i32> = HashMap::new();

    for (i, row) in schematic.iter().enumerate() {
        println!("-- LINE #{} --", i+1);
        let mut gear = (0,0);
        let mut num_string: String = "".to_string();

        for (j, ch) in row.iter().enumerate() {
            if ch.is_alphanumeric() {
                num_string += &ch.to_string();

                if gear == (0,0) {
                    // check around for gear
                    'outer: for k in max(i.saturating_sub(1),0) ..= min(schematic.len()-1, i.saturating_add(1)) {
                        for l in max(j.saturating_sub(1),0) ..= min(row.len()-1, j.saturating_add(1)) {
                            let c = &schematic[k][l];
                            if is_gear(c) {
                                gear = (k,l);
                                break 'outer;
                            }
                        }
                    }
                }
            }
            if (!ch.is_alphanumeric() || j == row.len() - 1) && !num_string.is_empty() { 
                //end of number
                if gear != (0,0) { // is connected to gear
                    let number: i32 = num_string.parse().unwrap();
                    match gear_map.get(&gear) {
                        Some(other_number) => {
                            // gear already found
                            println!("{} is connected to {}", number, other_number);
                            sum += other_number * number;
                        }
                        None => {
                            //new gear
                            gear_map.insert(gear, number);
                        },
                    }
                }
                num_string = "".to_string();
                gear = (0,0);
            }
        }
    }

    println!("Sum: {sum}");
    sum
}

fn main() {
    let file_path = "inputs/day3.txt";

    let schematic = match read_file_to_2d_vector(file_path) {
        Ok(matrix) => matrix,
        Err(err) => {
            eprintln!("Error reading file: {:?}", err);
            return;
        }
    };

    parse_schematic(schematic);
}
