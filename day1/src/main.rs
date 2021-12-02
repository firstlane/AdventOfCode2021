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
            part1(&mut input);
        },
        2 => {
            part2(&mut input);
        },
        _ => {
            panic!("Invalid part number: {}", part);
        },
    }
}

fn part1(input: &mut BufReader<File>) {
    let mut depth_increases = 0;
    let mut prev_depth: i32 = 0;

    for line in input.lines() {
        let depth = line.unwrap().parse::<i32>().unwrap();

        if depth_increases == 0 && prev_depth == 0 {
            prev_depth = depth;
            continue;
        }

        if depth > prev_depth {
            depth_increases += 1;
        }

        prev_depth = depth;
    }

    println!("Number of depth increases: {}", depth_increases);
}

fn part2(input: &mut BufReader<File>) {
    let mut depth_increases = 0;
    let window_size = 3;

    let lines = input.lines();
    let mut numbers = Vec::<i32>::new();
    for line in lines {
        let data = line.unwrap().parse::<i32>().unwrap();
        numbers.push(data);
    }

    for (i, _) in numbers.clone().into_iter().enumerate() {
        if numbers.len() < i + window_size + 1 {
            break;
        }

        let window1 = &numbers[i .. i + window_size];
        let window2 = &numbers[i + 1 .. i + window_size + 1];

        let window1_sum: i32 = window1.iter().sum();
        let window2_sum: i32 = window2.iter().sum();

        if window2_sum > window1_sum {
            depth_increases += 1;
        }
    }

    println!("Number of depth increases: {}", depth_increases);
}
