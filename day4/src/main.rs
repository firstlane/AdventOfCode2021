use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead, Error};
use std::env;
use std::result::Result;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file_path = String::new();
    let mut part = 1;

    match args.len() {
        3 => {
            // Assume first argument is part number,
            // and second is input file.
            part = args[1].parse::<i32>().unwrap();
            file_path = args[2].clone();
        },
        _ => {
            panic!("Invalid arguments");
        },
    }

    let file = match File::open(file_path) {
        Err(why) => panic!("couldn't open input file: {}", why),
        Ok(file) => file,
    };

    let mut input = BufReader::new(file);

    match part {
        1 => {
            let result = part1(&mut input);
            println!("Part 1: {}", result);
        },
        2 => {
            let result = part2(&mut input);
            println!("Part 2: {}", result);
        },
        _ => {
            panic!("Invalid part number: {}", part);
        },
    }
}



fn part1(input: &mut BufReader<File>) -> i32 {
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    let mut counts = Vec::<i32>::new();

    for line in input.lines() {
        let number = line.unwrap();
        let num_bits = number.len();

        if counts.len() < num_bits {
            counts.resize(num_bits, 0);
        }

        for (i, c) in number.chars().enumerate() {
            let bit = c.to_digit(2).unwrap();
            counts[i] += if bit > 0 { 1 } else { -1 };
        }
    }

    for (i, count) in counts.iter().enumerate() {
        if *count > 0 {
            gamma_rate += 2i32.pow((counts.len() - i - 1) as u32);
        }
        else {
            epsilon_rate += 2i32.pow((counts.len() - i - 1) as u32);
        }
    }

    return gamma_rate * epsilon_rate;
}

fn part2(input: &mut BufReader<File>) -> i32 {
    let mut oxygen_rating = 0;
    let mut co2_rating = 0;

    let mut numbers = Vec::<String>::new();
    for line in input.lines() {
        let number = line.unwrap();
        numbers.push(number);
    }

    let oxygen_rating = calculate_oxygen_rating(&numbers);
    let co2_rating = calculate_co2_rating(&numbers);

    println!("oxygen rating: {}", oxygen_rating);
    println!("co2 rating: {}", co2_rating);

    return oxygen_rating * co2_rating;
}

fn calculate_oxygen_rating(input: &Vec<String>) -> i32 {
    let mut numbers = input.clone();
    let num_bits = numbers[0].len();

    for i in 0..num_bits {
        let mut count = 0;

        if numbers.len() == 1 {
            break;
        }

        //let mut remove = Vec::<usize>::new();
        for (j, number) in numbers.iter().enumerate() {
            let bit_b = number.as_bytes()[i];
            let bit_c = bit_b as char;
            let bit = bit_c.to_digit(2).unwrap();

            if bit != 1 {
                //remove.push(i);
                count -= 1;
            }
            else {
                count += 1;
            }
        }

        if count >= 0 {
            // Keep only numbers with bit set to 1
            numbers.drain_filter(|num| {
                let bit = (num.as_bytes()[i] as char).to_digit(2).unwrap();
                if bit == 0 {
                    return true;
                }
                return false;
            });
        }
        else {
            // Keep only numbers with bit set to 0
            numbers.drain_filter(|num| {
                let bit = (num.as_bytes()[i] as char).to_digit(2).unwrap();
                if bit == 1 {
                    return true;
                }
                return false;
            });
        }
    }

    let num = i32::from_str_radix(numbers[0].as_str(), 2).unwrap();
    return num;
}

fn calculate_co2_rating(input: &Vec<String>) -> i32 {
    let mut numbers = input.clone();
    let num_bits = numbers[0].len();

    for i in 0..num_bits {
        let mut count = 0;

        if numbers.len() == 1 {
            break;
        }

        //let mut remove = Vec::<usize>::new();
        for (j, number) in numbers.iter().enumerate() {
            let bit_b = number.as_bytes()[i];
            let bit_c = bit_b as char;
            let bit = bit_c.to_digit(2).unwrap();

            if bit != 1 {
                //remove.push(i);
                count -= 1;
            }
            else {
                count += 1;
            }
        }

        if count >= 0 {
            // Keep only numbers with bit set to 0
            numbers.drain_filter(|num| {
                let bit = (num.as_bytes()[i] as char).to_digit(2).unwrap();
                if bit == 1 {
                    return true;
                }
                return false;
            });
        }
        else {
            // Keep only numbers with bit set to 1
            numbers.drain_filter(|num| {
                let bit = (num.as_bytes()[i] as char).to_digit(2).unwrap();
                if bit == 0 {
                    return true;
                }
                return false;
            });
        }
    }

    let num = i32::from_str_radix(numbers[0].as_str(), 2).unwrap();
    return num;
}
