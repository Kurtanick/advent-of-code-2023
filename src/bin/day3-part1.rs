use std::{char, i32};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::cmp::{min,max};

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

fn is_symbol(ch: &char) -> bool {
    ch.to_owned() != '.' && ch.is_ascii_punctuation()
}

fn parse_schematic(schematic: Vec<Vec<char>>) -> i32 {
    let mut sum = 0;

    for (i, row) in schematic.iter().enumerate() {
        println!("-- LINE #{} --", i+1);
        let mut valid = false;
        let mut num_string: String = "".to_string();

        for (j, ch) in row.iter().enumerate() {
            if ch.is_alphanumeric() {
                num_string += &ch.to_string();

                if !valid {
                    // check around for symbol
                    'outer: for k in max(i.saturating_sub(1),0) ..= min(schematic.len()-1, i.saturating_add(1)) {
                        for l in max(j.saturating_sub(1),0) ..= min(row.len()-1, j.saturating_add(1)) {
                            let c = &schematic[k][l];
                            if is_symbol(c) {
                                valid = true;
                                break 'outer;
                            }
                        }
                    }
                }
            } else if !num_string.is_empty() { 
                //end of number
                if valid {
                    println!("{}", num_string);
                    let number: i32 = num_string.parse().unwrap();
                    sum += number;
                }
                num_string = "".to_string();
                valid = false;
            }

            if j == row.len() - 1 { //end of the row
                if valid {
                    println!("{}", num_string);
                    let number: i32 = num_string.parse().unwrap();
                    sum += number;
                }
                num_string = "".to_string();
                valid = false;
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
