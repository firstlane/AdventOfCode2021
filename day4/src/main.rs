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

struct BingoBoard {
    numbers: [[i32; 5]; 5],
    marked: [[bool; 5]; 5],
    bingo: bool
}

impl BingoBoard {
    fn new() -> Self {
        Self {
            numbers: [[0; 5]; 5],
            marked: [[false; 5]; 5],
            bingo: false,
        }
    }

    fn clear(&mut self) {
        self.numbers = [[0; 5]; 5];
        self.marked = [[false; 5]; 5];
        self.bingo = false;
    }

    fn empty(&self) -> bool {
        for i in 0..self.numbers.len() {
            for j in 0..self.numbers[i].len() {
                if self.numbers[i][j] > 0 {
                    return false;
                }
            }
        }

        return true;
    }

    fn contains(&self, value: i32) -> bool {
        for i in 0..self.numbers.len() {
            for j in 0..self.numbers[i].len() {
                if self.numbers[i][j] == value {
                    return true;
                }
            }
        }

        return false;
    }

    fn marked(&self, row: usize, col: usize) -> bool {
        return self.marked[row][col];
    }

    fn mark(&mut self, value: i32) {
        for i in 0..self.numbers.len() {
            for j in 0..self.numbers[i].len() {
                if self.numbers[i][j] == value {
                    self.marked[i][j] = true;
                }
            }
        }
    }

    fn check_bingo(&mut self) -> bool {
        let row_length = self.marked.len();
        let col_length = self.marked[0].len();

        // Check each row
        for i in 0..row_length {
            let mut win = true;
            for j in 0..col_length {
                if !self.marked[i][j] {
                    win = false;
                    break;
                }
            }

            if win {
                self.bingo = true;
                return true;
            }
        }

        // Check each column
        for i in 0..col_length {
            let mut win = true;
            for j in 0..row_length {
                if !self.marked[j][i] {
                    win = false;
                    break;
                }
            }

            if win {
                self.bingo = true;
                return true;
            }
        }

        return false;
    }

    fn is_bingo(&self) -> bool {
        self.bingo
    }

    fn sum_unmarked(&self) -> i32 {
        let mut sum = 0;
        for i in 0..self.marked.len() {
            for j in 0..self.marked[i].len() {
                if !self.marked[i][j] {
                    sum += self.numbers[i][j];
                }
            }
        }

        return sum;
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

fn print_board(board: &BingoBoard) {
    for i in 0..board.numbers.len() {
        for j in 0..board.numbers[i].len() {
            print!("{number:>width$}", number=board.numbers[i][j], width=3);
        }
        println!("");
    }
}

fn print_boards(boards: &Vec<BingoBoard>) {
    for board in boards {
        print_board(board);
        println!("");
    }
}

fn part1(input: &mut BufReader<File>) -> i32 {
    let (mut numbers, mut boards) = parse_input(input);

    println!("{:?}", numbers);
    for number in numbers {
        for board in &mut boards {
            board.mark(number);
            //println!("Marked {}", number);
        }

        if number == 24 {
            print_boards(&boards);
        }

        for board in &mut boards {
            if board.check_bingo() {
                println!("{} * {}", board.sum_unmarked(), number);
                return board.sum_unmarked() * number;
            }
        }
    }

    return 0;
}

fn part2(input: &mut BufReader<File>) -> i32 {
    let (mut numbers, mut boards) = parse_input(input);

    println!("{:?}", numbers);
    for number in numbers {
        for board in &mut boards {
            board.mark(number);
        }

        let mut last_board_index = 0;
        for (i, board) in &mut boards.iter_mut().enumerate() {
            if !board.is_bingo() {
                board.check_bingo();

                if board.is_bingo() {
                    last_board_index = i;
                }
            }
        }

        if boards.iter().all(|board| board.is_bingo()) {
            let last_board = &boards[last_board_index];
            println!("{}", number);
            println!("{}", last_board_index);
            return last_board.sum_unmarked() * number;
        }
    }

    return 0;
}

