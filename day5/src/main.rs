use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead, Error};
use std::env;
use std::result::Result;

#[derive(PartialEq)]
struct Args {
    part_number: i8,
    input_file: String,
}

impl Args {
    fn new() -> Self {
        Self {
            part_number: 1,
            input_file: "".to_string(),
        }
    }
}

fn parse_args(args: &Vec<String>) -> Args {
    if args.contains(&"--help".to_string()) {
        println!("Usage:");
        println!("-p, --part: part number, 1 or 2");
        println!("-f, --file: input file");
        return Args::new();
    }

    let mut params = Args::new();

    let found_part = args.iter().position(|&p| p == "-p" || p == "--part");
    match found_part {
        None => println!(r#"Failed to find "--part" argument"#),
        Some(index) => {
            if index < args.len() {
                params.part_number = args[index + 1].parse::<i8>().expect(r#"Failed to parse "--part" value"#);
            }
        },
    }

    let found_file = args.iter().position(|&p| p == "-f" || p == "--file");
    match found_file {
        None => println!(r#"Failed to find "--file" argument"#),
        Some(index) => {
            if index < args.len() {
                params.input_file = args[index + 1];
            }
        },
    }

    params
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let params = parse_args(&args);
    if params == Args::new() {
        return
    }

    let file = match File::open(params.input_file) {
        Err(why) => panic!("couldn't open input file: {}", why),
        Ok(file) => file,
    };

    let mut input = BufReader::new(file);

    match params.part_number {
        1 => {
            let result = part1(&mut input);
            println!("Part 1: {}", result);
        },
        2 => {
            let result = part2(&mut input);
            println!("Part 2: {}", result);
        },
        _ => {
            panic!("Invalid part number: {}", params.part_number);
        },
    }
}

fn parse_input(input: &mut BufReader<File>) -> (Vec::<i32>, Vec::<BingoBoard>) {
    let bingo_numbers_str = input.lines().nth(0).unwrap().unwrap();

    let bingo_numbers_str = bingo_numbers_str.split(',');
    let mut bingo_numbers = Vec::<i32>::new();
    for number in bingo_numbers_str {
        bingo_numbers.push(number.parse::<i32>().unwrap());
    }

    let mut bingo_boards = Vec::<BingoBoard>::new();
    let mut bingo_board = BingoBoard::new();
    let mut row = 0;
    for line in input.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            if row > 0 {
                bingo_boards.push(bingo_board);
                bingo_board = BingoBoard::new();
            }

            row = 0;
            continue;
        }

        let row_data = line.split(' ');
        let mut column = 0;
        for c in row_data {
            let num = c.trim();
            if num.len() > 0 {
                let num = num.parse::<i32>().unwrap();
                bingo_board.numbers[row][column] = num;
                column += 1;
            }
        }

        row += 1;
    }

    if !bingo_board.empty() {
        bingo_boards.push(bingo_board);
    }

    print_boards(&bingo_boards);

    return (bingo_numbers, bingo_boards);
}

fn part1(input: &mut BufReader<File>) -> i32 {
    return 0;
}

fn part2(input: &mut BufReader<File>) -> i32 {
    return 0;
}

