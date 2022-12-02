use std::{fs::{File}, io::{BufReader, BufRead}};

struct Play {
    me: Shape,
    opponent: Shape,
}

enum Result {
    Win,
    Loss,
    Draw
}

impl Result {
    fn score(&self) -> u32 {
        match self {
            Result::Win => 6,
            Result::Loss => 0,
            Result::Draw => 3,
        }
    }

    fn from_symbol(symbol: char) -> Self {
        match symbol {
            'X' => Self::Loss,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!("Invalid symbol: {symbol}"),
        }
    }
}

impl Play {
    fn from_line_of_shape_and_result(line: &str) -> Self {
        let split_line = line.split(' ').collect::<Vec<_>>();
        let opponent = Shape::from_symbol(split_line[0].chars().take(1).last().unwrap_or_else(|| panic!("Invalid symbol: {}", split_line[0])));
        let my_result = Result::from_symbol(split_line[1].chars().take(1).last().unwrap_or_else(|| panic!("Invalid symbol: {}", split_line[1])));
        let me = Shape::from_opponent_and_my_result(&opponent, &my_result);
        Self {me, opponent}
    }

    fn from_line_of_shapes(line: &str) -> Self {
        let split_line = line.split(' ').collect::<Vec<_>>();
        let opponent = Shape::from_symbol(split_line[0].chars().take(1).last().unwrap_or_else(|| panic!("Invalid symbol: {}", split_line[0])));
        let me = Shape::from_symbol(split_line[1].chars().take(1).last().unwrap_or_else(|| panic!("Invalid symbol: {}", split_line[1])));
        Self { me, opponent }
    }

    fn result(&self) -> Result {
        match self.me {
            Shape::Rock => match self.opponent {
                Shape::Rock => Result::Draw,
                Shape::Paper => Result::Loss,
                Shape::Scissors => Result::Win,
            },
            Shape::Paper => match self.opponent {
                Shape::Rock => Result::Win,
                Shape::Paper => Result::Draw,
                Shape::Scissors => Result::Loss,
            },
            Shape::Scissors => match self.opponent {
                Shape::Rock => Result::Loss,
                Shape::Paper => Result::Win,
                Shape::Scissors => Result::Draw,
            },
        }
    }

    fn score(&self) -> u32 {
        self.result().score() + self.me.score()
    }
}

#[derive(Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from_symbol(symbol: char) -> Self {
        match symbol {
            'A' => Shape::Rock,
            'X' => Shape::Rock,
            'B' => Shape::Paper,
            'Y' => Shape::Paper,
            'C' => Shape::Scissors,
            'Z' => Shape::Scissors,
            _ => panic!("Unidentified symbol: {symbol}"),
        }
    }

    fn score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn from_opponent_and_my_result(opponent_shape: &Shape, result: &Result) -> Self {
        match opponent_shape {
            Shape::Rock => match result {
                Result::Win => Self::Paper,
                Result::Loss => Self::Scissors,
                Result::Draw => Self::Rock,
            },
            Shape::Paper =>match result {
                Result::Win => Self::Scissors,
                Result::Loss => Self::Rock,
                Result::Draw => Self::Paper,
            },
            Shape::Scissors =>match result {
                Result::Win => Self::Rock,
                Result::Loss => Self::Paper,
                Result::Draw => Self::Scissors,
            },
        }
    }
}

#[test]
fn play_from_symbol_test() {
    assert_eq!(Shape::from_symbol('A'), Shape::Rock);
    assert_eq!(Shape::from_symbol('X'), Shape::Rock);
    assert_eq!(Shape::from_symbol('B'), Shape::Paper);
    assert_eq!(Shape::from_symbol('Y'), Shape::Paper);
    assert_eq!(Shape::from_symbol('C'), Shape::Scissors);
    assert_eq!(Shape::from_symbol('Z'), Shape::Scissors);
}

#[test]
#[should_panic(expected = "Unidentified symbol: M")]
fn play_from_symbol_panic_on_unknown_symbol_test() {
    Shape::from_symbol('M');
}

fn first_part() {
    let file = File::open("./src/input.txt").expect("Failed reading file");
    let reader = BufReader::new(file);

    let plays: Vec<Play> = reader.lines().map(|line| Play::from_line_of_shapes(line.unwrap().as_str())).collect();
    let score = plays.iter().fold(0, |acc, play| acc + play.score());

    println!("First part; Score is: {score}");
}

fn second_part() {
    let file = File::open("./src/input.txt").expect("Failed reading file");
    let reader = BufReader::new(file);

    let plays: Vec<Play> = reader.lines().map(|line| Play::from_line_of_shape_and_result(line.unwrap().as_str())).collect();
    let score = plays.iter().fold(0, |acc, play| acc + play.score());

    println!("Second part; Score is: {score}");
}

fn main() {
    first_part();
    second_part();
}

