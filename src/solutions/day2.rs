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

#[derive(Debug, PartialEq, Eq)]
enum Result {
    Win(Shape),
    Lose(Shape),
    Draw(Shape),
}

impl Result {
    pub fn new(pair: (Shape, Shape)) -> Self {
        let (input, response) = pair;
        if input == response {
            return Result::Draw(response);
        }
        if response == Shape::of(input as i32 + 1) {
            return Result::Win(response);
        }
        return Result::Lose(response);
    }
    pub fn from(shape: Shape, result: &str) -> Option<Self> {
        match result {
            "X" => Some(Result::Lose(Shape::of(shape as i32 + 2))),
            "Y" => Some(Result::Draw(shape)),
            "Z" => Some(Result::Win(Shape::of(shape as i32 + 1))),
            _ => None,
        }
    }
    pub fn get_points(&self) -> i32 {
        return match self {
            Result::Win(res) => 6 + res.points(),
            Result::Draw(res) => 3 + res.points(),
            Result::Lose(res) => 0 + res.points(),
        };
    }
}

pub fn solve(input: String) {
    part1(&input);
    part2(&input);
}

fn part2(input: &String) {
    let points: i32 = input
        .trim()
        .split("\n")
        .map(|round| {
            let (first, res) = round
                .split(" ")
                .collect_tuple()
                .expect("Invalid round entry");
            let shape = Shape::new(first).expect("Unable to convert input to shape");
            return Result::from(shape, res)
                .expect("Unable to get result from shape")
                .get_points();
        })
        .sum();

    println!("Total points from revised strategy guide {}", points);
}

fn part1(input: &String) {
    let points: i32 = input
        .trim()
        .split("\n")
        .map(|round| {
            let split_round = round
                .split(" ")
                .map(|i| Shape::new(i).expect("Invalid shape type"))
                .collect_tuple()
                .expect("Invalid round entry");
            return Result::new(split_round).get_points();
        })
        .sum();

    println!("Total points from strategy guide {}", points);
}
