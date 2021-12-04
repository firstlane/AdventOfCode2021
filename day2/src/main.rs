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
    let mut horiz = 0;
    let mut depth = 0;

    for line in input.lines() {
        let command = line.unwrap();
        let components: Vec<&str> = command.split(' ').collect();
        assert_eq!(components.len(), 2);

        let motion = components[0];
        let value = components[1].parse::<i32>().unwrap();

        match motion {
            "forward" => {
                horiz += value;
            },
            "up" => {
                depth -= value;
            },
            "down" => {
                depth += value;
            },
            _ => {
                panic!("Invalid command: {}", motion)
            },
        }
    }

    return horiz * depth;
}

fn part2(input: &mut BufReader<File>) -> i32 {
    let mut horiz = 0;
    let mut depth = 0;
    let mut aim   = 0;

    for line in input.lines() {
        let command = line.unwrap();
        let components: Vec<&str> = command.split(' ').collect();
        assert_eq!(components.len(), 2);

        let motion = components[0];
        let value = components[1].parse::<i32>().unwrap();

        match motion {
            "forward" => {
                horiz += value;
                depth += aim * value;
            },
            "up" => {
                aim -= value;
            },
            "down" => {
                aim += value;
            },
            _ => {
                panic!("Invalid command: {}", motion)
            },
        }
    }

    return horiz * depth;
}
