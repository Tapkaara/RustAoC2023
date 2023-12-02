use std::convert::TryInto;
use std::fs::File;
use std::io::{self, BufRead};

pub fn findDigits(lines: Result<io::Lines<io::BufReader<File>>, std::io::Error>) -> u32 {
    let mut first = 0;
    let mut last = 0;
    let mut sum = 0;

    if let Ok(lines) = lines {
        for line in lines {
            if let Ok(cs) = line {
                // Find first digit
                for i in cs.chars() {
                    if i.is_digit(10) {
                        first = i.to_digit(10).unwrap();
                        // println!("first: {}", first);
                        break;
                    }
                }

                // Find last digit
                for i in cs.chars().rev() {
                    if i.is_digit(10) {
                        last = i.to_digit(10).unwrap();
                        // println!("last: {}", last);
                        break;
                    }
                }
                sum += first * 10 + last;
                // println!("sum: {}", sum);
            }
        }
    }
    //println!("Total: {}", sum);
    return sum;
}

pub fn findNumbers(lines: Result<io::Lines<io::BufReader<File>>, std::io::Error>) -> i32 {
    let numberStrings: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut sum = 0;

    // Why can't take lines by reference and dereference here?
    if let Ok(text) = lines {
        for line in text {
            if let Ok(cs) = line {
                let mut first = 0;
                let mut last = 0;

                let mut count: i32 = 0;
                let mut index: i32 = cs.len().try_into().unwrap();

                for j in numberStrings.as_slice() {
                    // numberStrings starts at 1 so increment first
                    count += 1;
                    if let Some(v) = cs.find(j) {
                        if index > v.try_into().unwrap() {
                            first = count;
                            index = v.try_into().unwrap();
                        }
                        // println!("looking for: {}", j);
                        // println!("count: {}", count);
                        //  println!("first: {}", first);
                        //  println!("index: {}", index);
                    }
                }

                // Find first digit
                count = 0;
                for i in cs.chars() {
                    if i.is_digit(10) {
                        first = if count < index {
                            i.to_digit(10).unwrap().try_into().unwrap()
                        } else {
                            first
                        };

                        // println!("first: {}", first);
                        break;
                    }
                    count += 1;
                }

                count = 0;
                index = -1;
                for j in numberStrings.as_slice() {
                    // numberStrings starts at 1 so increment first
                    count += 1;
                    if let Some(v) = cs.rfind(j) {
                        if index < v.try_into().unwrap() {
                            last = count;
                            index = v.try_into().unwrap();
                            // println!("last alpha: {}", last);
                        }
                        // println!("looking for: {}", j);
                        // println!("count: {}", count);
                        // println!("first: {}", first);
                        // println!("index: {}", index);
                    }
                }

                // Find last digit
                count = 0;
                for i in cs.chars().rev() {
                    if i.is_digit(10) {
                        last = if (cs.len() as i32 - 1 - count) > index {
                            i.to_digit(10).unwrap().try_into().unwrap()
                        } else {
                            last
                        };
                        // println!("last: {}", last);
                        break;
                    }
                    count += 1;
                }
                sum += first * 10 + last;
                // println!("Pair                      : {} : {}", first * 10, last);
                // println!("Restart and sum: {}", sum);
            }
        }
    }
    // println!("Total: {}", sum);
    return sum;
}

fn main() -> Result<(), std::io::Error> {
    let file: File = File::open("res/input.txt")?;
    let lines: Result<io::Lines<io::BufReader<File>>, std::io::Error> =
        Ok(io::BufReader::new(file).lines());

    // println!(findDigits(lines));
    println!("Total: {}", findNumbers(lines));

    return Ok(());
}
