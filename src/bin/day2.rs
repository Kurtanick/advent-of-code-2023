use std::fs::*;
use std::io::*;
use std::cmp::max as max;

const TOTAL_RED: i32 =   12;
const TOTAL_GREEN: i32 = 13;
const TOTAL_BLUE: i32 =  14;

fn main() {
    let _ = part1();
}

fn part1() -> Result<()> {
    let f = File::open("inputs/day2.txt")?;
    let reader = BufReader::new(f);
    
    let mut result = 0i32;
    for l in reader.lines() {
        let line = l.unwrap();
        
        let id = parse_id(&line);

        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;

        let sets = line.get(line.find(":").unwrap() + 2 ..).unwrap().split(";");
        for set in sets {
            let colors = set.split(",");
            for color in colors {
                if color.contains("red") {
                    let count :i32 = parse_count(color);
                    max_red = max(max_red, count)
                } else if color.contains("blue") {
                    let count :i32 = parse_count(color);
                    max_blue = max(max_blue, count)
                } else if color.contains("green"){
                    let count :i32 = parse_count(color);
                    max_green = max(max_green, count)
                }
            }
        }

        //is set valid?
        if max_red <= TOTAL_RED && max_blue <= TOTAL_BLUE && max_green <= TOTAL_GREEN {
            result += id;    
        }

    }

    println!("Result: {}", result);


    Ok(())
}

fn parse_id(line: &str) -> i32 {
    let colon_index = line.find(":").unwrap();
    let id_string = line.get(5 .. colon_index).unwrap();
    id_string.parse().unwrap()
}

fn parse_count(color: &str) -> i32 {
    color.trim().split(" ").next().unwrap().parse().unwrap()
}