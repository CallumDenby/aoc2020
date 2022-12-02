use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn new(shape: &str) -> Option<Self> {
        match shape {
            "A" | "X" => Some(Shape::Rock),
            "B" | "Y" => Some(Shape::Paper),
            "C" | "Z" => Some(Shape::Scissors),
            _ => None,
        }
    }
    pub fn of(index: i32) -> Self {
        let idx = index.rem_euclid(3);
        match idx {
            0 => Shape::Rock,
            1 => Shape::Paper,
            2 => Shape::Scissors,
            _ => panic!("Should not reach here"),
        }
    }
    pub fn points(&self) -> i32 {
        (*self as i32) + 1
    }
}

const WIN: i32 = 6;
const DRAW: i32 = 3;
const LOSE: i32 = 0;

pub fn solve(input: String) {
    let (first, second) = input
        .trim()
        .split("\n")
        .map(|round| round.split(" ").collect_tuple().unwrap())
        .fold((0, 0), |(one, two), curr| {
            (one + part1(curr), two + part2(curr).unwrap_or(0))
        });

    println!("Part 1: {}", first);
    println!("Part 2: {}", second);
}

fn part1((first, second): (&str, &str)) -> i32 {
    let input = Shape::new(first).unwrap();
    let response = Shape::new(second).unwrap();
    let draw = if input == response { DRAW } else { 0 };
    let win = if response == Shape::of(input as i32 + 1) {
        WIN
    } else {
        0
    };
    return draw + win + response.points();
}

fn part2((first, second): (&str, &str)) -> Option<i32> {
    let input = Shape::new(first).unwrap();
    match second {
        "X" => Some(LOSE + Shape::of(input as i32 + 2).points()),
        "Y" => Some(DRAW + input.points()),
        "Z" => Some(WIN + Shape::of(input as i32 + 1).points()),
        _ => None,
    }
}
